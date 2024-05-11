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

#[cfg(test)]
mod config_tests {
    use lazy_static::lazy_static;
    use std::env;
    use std::sync::Mutex;
    use ws2mongo::config::Config;

    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_config_all_env_vars_set() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // Set environment variables for testing
        env::set_var("WEBSOCKET_URL", "ws://example.com");
        env::set_var("WEBSOCKET_API_KEY", "key123");
        env::set_var("WEBSOCKET_API_SECRET", "secret123");
        env::set_var("MONGODB_URI", "mongodb://localhost:27017");
        env::set_var("DATABASE_NAME", "testdb");
        env::set_var("COLLECTION_NAME", "testcollection");
        env::set_var("MONGODB_USER", "user");
        env::set_var("MONGODB_PASSWORD", "password");
        env::set_var("MONGODB_AUTH_SOURCE", "admin");
        env::set_var("MONGODB_AUTH_MECHANISM", "SCRAM-SHA-256");

        let config = Config::new();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.websocket_url, "ws://example.com");
        assert_eq!(config.websocket_api_key.unwrap(), "key123");
        assert_eq!(config.websocket_api_secret.unwrap(), "secret123");
        assert_eq!(config.mongodb_uri, "mongodb://localhost:27017");
        assert_eq!(config.database_name, "testdb");
        assert_eq!(config.collection_name, "testcollection");
        assert_eq!(config.mongodb_user.unwrap(), "user");
        assert_eq!(config.mongodb_password.unwrap(), "password");
        assert_eq!(config.mongodb_auth_source.unwrap(), "admin");
        assert_eq!(config.mongodb_auth_mechanism.unwrap(), "SCRAM-SHA-256");
    }

    #[test]
    fn test_config_missing_required_vars() {
        let _guard = ENV_MUTEX.lock().unwrap();
        env::remove_var("WEBSOCKET_URL");
        env::remove_var("MONGODB_URI");
        env::remove_var("DATABASE_NAME");
        env::remove_var("COLLECTION_NAME");

        let config = Config::new();
        assert!(config.is_err());
    }

    #[test]
    fn test_config_optional_vars_missing() {
        let _guard = ENV_MUTEX.lock().unwrap();
        env::set_var("WEBSOCKET_URL", "ws://example.com");
        env::set_var("MONGODB_URI", "mongodb://localhost:27017");
        env::set_var("DATABASE_NAME", "testdb");
        env::set_var("COLLECTION_NAME", "testcollection");

        // Optional variables not set
        env::remove_var("WEBSOCKET_API_KEY");
        env::remove_var("WEBSOCKET_API_SECRET");
        env::remove_var("MONGODB_USER");
        env::remove_var("MONGODB_PASSWORD");
        env::remove_var("MONGODB_AUTH_SOURCE");
        env::remove_var("MONGODB_AUTH_MECHANISM");

        let config = Config::new();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.websocket_url, "ws://example.com");
        assert!(config.websocket_api_key.is_none());
        assert!(config.websocket_api_secret.is_none());
        assert_eq!(config.mongodb_uri, "mongodb://localhost:27017");
        assert_eq!(config.database_name, "testdb");
        assert_eq!(config.collection_name, "testcollection");
        assert!(config.mongodb_user.is_none());
        assert!(config.mongodb_password.is_none());
        assert!(config.mongodb_auth_source.is_none());
        assert!(config.mongodb_auth_mechanism.is_none());
    }
}
