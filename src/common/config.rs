use color_eyre::eyre::Result;
use figment::{providers::Env, Figment};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub environment: Environment,

    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,

    pub service_host: String,
    pub service_port: u32,

    pub logging_level: String,
    pub jwt_secret_key: String,

    pub cors: CorsConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allow_origin: String,
    pub allow_methods: String,
    pub allow_headers: String,
    pub allow_credentials: bool,
    pub max_age: u64,
}

impl CorsConfig {
    pub fn parse_origins(&self) -> Vec<String> {
        self.parse_separator_helper(&self.allow_origin)
    }

    pub fn parse_methods(&self) -> Vec<String> {
        self.parse_separator_helper(&self.allow_methods)
    }

    pub fn parse_headers(&self) -> Vec<String> {
        self.parse_separator_helper(&self.allow_headers)
    }

    fn parse_separator_helper(&self, paramters: &str) -> Vec<String> {
        paramters
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        // Use __ (double underscore) as the delimiter for nested structures
        // Example: CORS__ALLOW_ORIGIN becomes cors.allow_origin
        let config: Config = Figment::new()
            .merge(Env::prefixed("P2P_APP_").split("__"))
            .extract()?;

        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.service_host, self.service_port)
    }

    pub async fn create_database_pool(&self) -> Result<DatabaseConnection> {
        setup_database(self).await
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
