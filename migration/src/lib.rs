pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241209_000002_create_roles;
mod m20241209_000003_drop_is_admin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241209_000002_create_roles::Migration),
            Box::new(m20241209_000003_drop_is_admin::Migration),
        ]
    }
}
