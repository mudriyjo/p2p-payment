use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const ADMIN_ROLE_ID: &str = "878c19c6-643b-4a57-98f1-a60786a38a92";
const SUPPORT_ROLE_ID: &str = "e79d6652-5efb-43ae-9565-04b3d3fcfc0f";
const RISK_ROLE_ID: &str = "48cd5981-0e75-4329-8e1d-57681e8715db";
const FINANCE_ROLE_ID: &str = "2e457833-9393-4a8f-9c0e-4314e1425312";
const USER_ROLE_ID: &str = "eec86d00-495c-490c-b151-b9d33672a681";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Create roles table
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(uuid(Roles::RoleId).primary_key())
                    .col(string(Roles::RoleName).unique_key().not_null())
                    .col(string_null(Roles::RoleDescription))
                    .col(timestamp_with_time_zone(Roles::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Roles::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        // Step 2: Seed default roles with fixed UUIDs
        let now = chrono::Utc::now();
        let now_str = now.to_rfc3339();

        // Use raw SQL to insert roles with specific UUIDs
        let insert_roles = format!(
            r#"
            INSERT INTO roles (role_id, role_name, role_description, created_at, updated_at)
            VALUES
                ('{}', 'Admin', 'Full system access with all permissions', '{}', '{}'),
                ('{}', 'Support', 'Customer support and assistance permissions', '{}', '{}'),
                ('{}', 'Risk', 'Risk management and fraud prevention permissions', '{}', '{}'),
                ('{}', 'Finance', 'Financial operations and reporting permissions', '{}', '{}'),
                ('{}', 'User', 'Standard user access', '{}', '{}')
            ON CONFLICT (role_id) DO NOTHING
            "#,
            ADMIN_ROLE_ID, now_str, now_str,
            SUPPORT_ROLE_ID, now_str, now_str,
            RISK_ROLE_ID, now_str, now_str,
            FINANCE_ROLE_ID, now_str, now_str,
            USER_ROLE_ID, now_str, now_str
        );

        manager
            .get_connection()
            .execute_unprepared(&insert_roles)
            .await?;

        // Step 3: Add role_id column to users (NULLABLE initially for safe migration)
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(uuid_null(Users::RoleId))
                    .to_owned(),
            )
            .await?;

        // Step 4: Add foreign key constraint
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_users_role_id")
                    .from(Users::Table, Users::RoleId)
                    .to(Roles::Table, Roles::RoleId)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Step 5: Data migration - Set role_id based on is_admin
        let update_sql = format!(
            r#"
            UPDATE users
            SET role_id = CASE
                WHEN is_admin = true THEN '{}'::uuid
                ELSE '{}'::uuid
            END
            WHERE role_id IS NULL
            "#,
            ADMIN_ROLE_ID, USER_ROLE_ID
        );

        manager
            .get_connection()
            .execute_unprepared(&update_sql)
            .await?;

        // Step 6: Make role_id NOT NULL after migration
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(uuid(Users::RoleId).not_null())
                    .to_owned(),
            )
            .await?;

        // Step 7: Set default value for new users
        let set_default = format!(
            r#"
            ALTER TABLE users
            ALTER COLUMN role_id SET DEFAULT '{}'::uuid
            "#,
            USER_ROLE_ID
        );

        manager
            .get_connection()
            .execute_unprepared(&set_default)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rollback in reverse order

        // Remove default value
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE users ALTER COLUMN role_id DROP DEFAULT")
            .await?;

        // Make role_id nullable before dropping
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(uuid_null(Users::RoleId))
                    .to_owned(),
            )
            .await?;

        // Drop foreign key
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_users_role_id")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        // Drop role_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::RoleId)
                    .to_owned(),
            )
            .await?;

        // Drop roles table
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    RoleId,
    RoleName,
    RoleDescription,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    RoleId,
}
