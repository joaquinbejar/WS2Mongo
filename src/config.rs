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

use serde_json::json;
use std::env;
use thiserror::Error;
use crate::constants::{*};

/// Represents the configuration options for the application.
#[derive(Debug, Clone)]
pub struct Config {
    /// The WebSocket URL for connecting to the server.
    pub websocket_url: String,

    /// Optional API key for WebSocket authentication.
    pub websocket_api_key: Option<String>,

    /// Optional API secret for WebSocket authentication.
    pub websocket_api_secret: Option<String>,

    /// The MongoDB URI for connecting to the database.
    pub mongodb_uri: String,

    /// The name of the database to use.
    pub database_name: String,

    /// The name of the collection within the database.
    pub collection_name: String,

    /// Optional username for MongoDB authentication.
    pub mongodb_user: Option<String>,

    /// Optional password for MongoDB authentication.
    pub mongodb_password: Option<String>,

    /// Optional authentication source for MongoDB.
    pub mongodb_auth_source: String,

    /// Optional authentication mechanism for MongoDB.
    pub mongodb_auth_mechanism: String,
}

/// An enum representing various errors that can occur during configuration.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error indicating that a required environment variable is missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),
}

impl Config {
    /// Creates a new `Config` instance by reading environment variables.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if a required environment variable is missing.
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Config {
            websocket_url: Self::get_env_var_or_default("WEBSOCKET_URL", WEBSOCKET_URL.to_string()),
            websocket_api_key: env::var("WEBSOCKET_API_KEY").ok(),
            websocket_api_secret: env::var("WEBSOCKET_API_SECRET").ok(),
            mongodb_uri: Self::get_env_var_or_default("MONGODB_URI", MONGODB_URI.to_string()),
            database_name: Self::get_env_var_or_error("DATABASE_NAME")?,
            collection_name: Self::get_env_var_or_error("COLLECTION_NAME")?,
            mongodb_user: env::var("MONGODB_USER").ok(),
            mongodb_password: env::var("MONGODB_PASSWORD").ok(),
            mongodb_auth_source: Self::get_env_var_or_default(("MONGODB_AUTH_SOURCE"), MONGODB_AUTH_SOURCE.to_string()),
            mongodb_auth_mechanism: Self::get_env_var_or_default(("MONGODB_AUTH_MECHANISM"), MONGODB_AUTH_MECHANISM.to_string())
        })
    }

    /// Gets the value of an environment variable or returns a default value if the variable is not set.
    ///
    /// # Arguments
    ///
    /// * `var_name` - The name of the environment variable.
    /// * `default_value` - The default value to use if the environment variable is not set.
    fn get_env_var_or_default(var_name: &str, default_value: String) -> String {
        env::var(var_name).unwrap_or(default_value)
    }

    /// Gets the value of an environment variable or returns an error if the variable is not set.
    ///
    /// # Arguments
    ///
    /// * `var_name` - The name of the environment variable.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError::MissingEnvVar` if the environment variable is not set.
    fn get_env_var_or_error(var_name: &str) -> Result<String, ConfigError> {
        env::var(var_name).map_err(|_| ConfigError::MissingEnvVar(var_name.to_string()))
    }

    /// Serializes the configuration to a JSON string.
    ///
    /// # Returns
    ///
    /// Returns a JSON string representation of the configuration.
    ///
    /// # Errors
    ///
    /// Returns a `serde_json::Error` if serialization fails.
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
