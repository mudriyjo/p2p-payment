use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let db_url = std::env::var("P2P_APP_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("Neither P2P_APP_DATABASE_URL nor DATABASE_URL is set");
    
    std::env::set_var("DATABASE_URL", db_url);

    cli::run_cli(migration::Migrator).await;
}
