/*******************************************************************************
 * Copyright (c) 2024.
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General
 * Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>..
 ******************************************************************************/

/******************************************************************************
   Author:
   Email: jb@taunais.com
   Date: 11/5/24
******************************************************************************/

use std::env;
use serde_json::json;
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
            websocket_url: env::var("WEBSOCKET_URL")
                .map_err(|_| ConfigError::MissingEnvVar("WEBSOCKET_URL".to_string()))?,
            websocket_api_key: env::var("WEBSOCKET_API_KEY").ok(),
            websocket_api_secret: env::var("WEBSOCKET_API_SECRET").ok(),
            mongodb_uri: env::var("MONGODB_URI")
                .map_err(|_| ConfigError::MissingEnvVar("MONGODB_URI".to_string()))?,
            database_name: env::var("DATABASE_NAME")
                .map_err(|_| ConfigError::MissingEnvVar("DATABASE_NAME".to_string()))?,
            collection_name: env::var("COLLECTION_NAME")
                .map_err(|_| ConfigError::MissingEnvVar("COLLECTION_NAME".to_string()))?,
            mongodb_user: env::var("MONGODB_USER").ok(),
            mongodb_password: env::var("MONGODB_PASSWORD").ok(),
            mongodb_auth_source: env::var("MONGODB_AUTH_SOURCE").ok(),
            mongodb_auth_mechanism: env::var("MONGODB_AUTH_MECHANISM").ok(),
        })
    }
    pub fn print_as_json(&self) -> serde_json::Result<String> {
        let json_config = json!({
            "WEBSOCKET_URL": self.websocket_url,
            "WEBSOCKET_API_KEY": self.websocket_api_key,
            "WEBSOCKET_API_SECRET": self.websocket_api_secret,
            "MONGODB_URI": self.mongodb_uri,
            "DATABASE_NAME": self.database_name,
            "COLLECTION_NAME": self.collection_name,
            "MONGODB_USER": self.mongodb_user,
            "MONGODB_PASSWORD": self.mongodb_password,
            "MONGODB_AUTH_SOURCE": self.mongodb_auth_source,
            "MONGODB_AUTH_MECHANISM": self.mongodb_auth_mechanism,
        });
        serde_json::to_string_pretty(&json_config)
    }
}
