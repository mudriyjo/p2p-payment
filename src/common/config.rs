use color_eyre::eyre::Result;
use figment::{Figment, providers::Env};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,

    pub service_host: String,
    pub service_port: u32,

    pub logging_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config: Config = Figment::new().merge(Env::prefixed("P2P_APP_")).extract()?;

        Ok(config)
    }
}

pub async fn setup_database(config: &Config) -> Result<DatabaseConnection> {
    let mut option = ConnectOptions::new(config.database_url.clone());
    option
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .sqlx_logging(true)
        .sqlx_logging_level(
            config
                .logging_level
                .parse()
                .unwrap_or(tracing::log::LevelFilter::Info),
        );

    let pool = Database::connect(option).await?;

    Ok(pool)
}
