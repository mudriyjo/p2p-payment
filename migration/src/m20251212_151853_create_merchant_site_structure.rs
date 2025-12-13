use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Create merchant table
        manager
            .create_table(
                Table::create()
                    .table(Merchant::Table)
                    .if_not_exists()
                    .col(uuid(Merchant::Id).primary_key())
                    .col(string(Merchant::Name).unique_key().not_null())
                    .col(string_null(Merchant::Description))
                    .col(string(Merchant::Status).not_null())
                    .col(timestamp_with_time_zone(Merchant::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Merchant::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        // Step 2: Create site table
        manager
            .create_table(
                Table::create()
                    .table(Site::Table)
                    .if_not_exists()
                    .col(uuid(Site::Id).primary_key())
                    .col(uuid(Site::MerchantId).not_null())
                    .col(string(Site::Name).unique_key().not_null())
                    .col(string(Site::Url).unique_key().not_null())
                    .col(string(Site::CallbackUrl).not_null())
                    .col(string(Site::RedirectSuccessUrl).not_null())
                    .col(string(Site::RedirectFailUrl).not_null())
                    .col(string(Site::Status).not_null())
                    .col(timestamp_with_time_zone(Site::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Site::UpdatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_merchant_site_id")
                        .from(Site::Table, Site::MerchantId)
                        .to(Merchant::Table, Merchant::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

       
        // Step 3: Create site table
        manager
            .create_table(
                Table::create()
                    .table(SiteCredentials::Table)
                    .if_not_exists()
                    .col(uuid(SiteCredentials::Id).primary_key())
                    .col(uuid(SiteCredentials::SiteId).not_null())
                    .col(string(SiteCredentials::PublicKey).unique_key().not_null())
                    .col(string(SiteCredentials::SecretKey).unique_key().not_null())
                    .col(
                        ColumnDef::new(SiteCredentials::AllowedIps)
                            .array(ColumnType::String(StringLen::None)) 
                            .not_null()
                    )
                    .col(string(SiteCredentials::IsActive).not_null())
                    .col(timestamp_with_time_zone(SiteCredentials::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_site_credentials_site_id")
                        .from(SiteCredentials::Table, SiteCredentials::SiteId)
                        .to(Site::Table, Site::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Step 4: Add JOIN indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_site_merchant_id")
                    .table(Site::Table)
                    .col(Site::MerchantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_site_credentials_site_id")
                    .table(SiteCredentials::Table)
                    .col(SiteCredentials::SiteId)
                    .to_owned(),
            )
            .await?;

        // Step 4: Add Search indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_merchant_name")
                    .table(Merchant::Table)
                    .col(Merchant::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_site_name")
                    .table(Site::Table)
                    .col(Site::Name)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(SiteCredentials::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Site::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Merchant::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Merchant {
    Table,
    Id,
    Name,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Site {
    Table,
    Id,
    MerchantId,
    Name,
    Url,
    CallbackUrl,
    RedirectSuccessUrl,
    RedirectFailUrl,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SiteCredentials {
    Table,
    Id,
    SiteId,
    PublicKey,
    SecretKey,
    AllowedIps,
    IsActive,
    CreatedAt,
}