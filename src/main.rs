mod common;

use axum::{
    Extension, Router,
    routing::{get},
};
use color_eyre::eyre::Result;
use migration::{Migrator, MigratorTrait};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::config::{Config, setup_database};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let fmt = tracing_subscriber::fmt::layer();
    let filter = tracing_subscriber::EnvFilter::from_default_env();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .with(ErrorLayer::default())
        .init();

    let config: Config = Config::from_env()?;
    let connection_pool = setup_database(&config).await?;

    Migrator::up(&connection_pool, None).await?;

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(Extension(connection_pool.clone()));

    let server_start_string = format!("{}:{}", config.service_host, config.service_port);
    let listener = tokio::net::TcpListener::bind(server_start_string)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> String {
    "Hello, world!".to_string()
}
