use color_eyre::eyre::Result;
use sea_orm::DatabaseConnection;
use migration::MigratorTrait;
use std::sync::Arc;
use crate::common::{AppState, Config};

pub async fn initialize_app() -> Result<Arc<AppState>> {

    tracing::info!("Loading configuration...");
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(config.logging_level.parse().unwrap_or(tracing::Level::INFO).into()),
        )
        .init();

    tracing::info!("Starting application initialization...");

    tracing::info!("Connecting to database...");
    let db = config.create_database_pool().await?;
    tracing::info!("Database connection established");

    verify_database_connection(&db).await?;

    tracing::info!("Running database migrations...");
    migration::Migrator::up(&db, None).await?;
    tracing::info!("Database migrations applied successfully");

    tracing::info!("Creating application state...");
    let state = Arc::new(AppState::new(db, config));
    tracing::info!("Application state created successfully");

    tracing::info!("Application initialization complete");
    Ok(state)
}


async fn verify_database_connection(db: &DatabaseConnection) -> Result<()> {
    use sea_orm::ConnectionTrait;

    db.execute(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT 1".to_owned(),
    ))
    .await?;

    tracing::info!("Database connection verified");
    Ok(())
}