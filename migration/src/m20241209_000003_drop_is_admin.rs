use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const ADMIN_ROLE_ID: &str = "878c19c6-643b-4a57-98f1-a60786a38a92";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop is_admin column from users table
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::IsAdmin)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add is_admin column back with default false
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(boolean(Users::IsAdmin).not_null().default(false))
                    .to_owned(),
            )
            .await?;

        // Restore is_admin from role_id (Admin role -> true, others -> false)
        let restore_sql = format!(
            r#"
            UPDATE users
            SET is_admin = CASE
                WHEN role_id = '{}'::uuid THEN true
                ELSE false
            END
            "#,
            ADMIN_ROLE_ID
        );

        manager
            .get_connection()
            .execute_unprepared(&restore_sql)
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    IsAdmin,
}
