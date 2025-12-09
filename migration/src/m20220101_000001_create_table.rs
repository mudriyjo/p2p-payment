use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(uuid(Users::Id).primary_key())
                    .col(string(Users::Username).unique_key().not_null())
                    .col(string(Users::Email).unique_key().not_null())
                    .col(string(Users::PasswordHash).not_null())
                    .col(boolean(Users::IsActive).not_null().default(true))
                    .col(boolean(Users::IsAdmin).not_null().default(false))
                    .col(timestamp_with_time_zone(Users::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Users::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    IsActive,
    IsAdmin,
    CreatedAt,
    UpdatedAt,
}
