use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub websocket_url: String,
    pub websocket_api_key: Option<String>,
    pub websocket_api_secret: Option<String>,
    pub mongodb_uri: String,
    pub database_name: String,
    pub collection_name: String,
    pub mongodb_user: Option<String>,
    pub mongodb_password: Option<String>,
    pub mongodb_auth_source: Option<String>,
    pub mongodb_auth_mechanism: Option<String>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Config {
            websocket_url: env::var("WEBSOCKET_URL").map_err(|_| ConfigError::MissingEnvVar("WEBSOCKET_URL".to_string()))?,
            websocket_api_key: env::var("WEBSOCKET_API_KEY").ok(),
            websocket_api_secret: env::var("WEBSOCKET_API_SECRET").ok(),
            mongodb_uri: env::var("MONGODB_URI").map_err(|_| ConfigError::MissingEnvVar("MONGODB_URI".to_string()))?,
            database_name: env::var("DATABASE_NAME").map_err(|_| ConfigError::MissingEnvVar("DATABASE_NAME".to_string()))?,
            collection_name: env::var("COLLECTION_NAME").map_err(|_| ConfigError::MissingEnvVar("COLLECTION_NAME".to_string()))?,
            mongodb_user: env::var("MONGODB_USER").ok(),
            mongodb_password: env::var("MONGODB_PASSWORD").ok(),
            mongodb_auth_source: env::var("MONGODB_AUTH_SOURCE").ok(),
            mongodb_auth_mechanism: env::var("MONGODB_AUTH_MECHANISM").ok()
        })
    }
}
