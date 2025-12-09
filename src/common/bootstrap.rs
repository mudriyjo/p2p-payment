use color_eyre::eyre::Result;
use sqlx::PgPool;
use std::sync::Arc;
use crate::common::{AppState, Config};

pub async fn initialize_app() -> Result<Arc<AppState>> {
    
    tracing::info!("Loading configuration...");
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(config.tracing.level.parse().unwrap_or(tracing::Level::INFO).into()),
        )
        .init();

    tracing::info!("Starting application initialization...");

    tracing::info!("Connecting to database...");
    let pool = Config::create_database_pool(&config).await?;
    tracing::info!("Database connection established");

    verify_database_connection(&pool).await?;

    tracing::info!("Creating application state...");
    let state = Arc::new(AppState::new(pool, config.jwt_secret_key.clone()));
    tracing::info!("Application state created successfully");

    tracing::info!("Application initialization complete");
    Ok(state)
}


async fn verify_database_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;

    tracing::info!("Database connection verified");
    Ok(())
}