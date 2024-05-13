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
   Date: 12/5/24
******************************************************************************/

use std::env;
use std::sync::Arc;
use tungstenite::Message;
use ws2mongo::config::Config;
use ws2mongo::mongodb::MongoClient;

#[tokio::main]
async fn main() {
    // Configuring the environment variables (replace these with actual environment variable settings or direct assignments)
    env::set_var("WEBSOCKET_URL", "ws://localhost:5678");
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    env::set_var("DATABASE_NAME", "test");
    env::set_var("COLLECTION_NAME", "test_data");
    env::set_var("MONGODB_AUTH_SOURCE", "admin");
    env::set_var("MONGODB_AUTH_MECHANISM", "SCRAM-SHA-256");

    let config = Config::new().expect("Failed to load config");
    // let client = MongoClient::new(config).await.expect("Failed to create MongoDB client");

    let message = r#"{"type": "subscribe", "symbol": "BTCUSD"}"#;

    let instance = MongoClient::new(config).await.unwrap();
    let instance_clone = Arc::clone(&instance);
    let message = Message::Text(String::from(message));

    instance_clone.enqueue(message).await.unwrap();

    // sleep for 10 seconds to keep the program running and allow the message to be inserted
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
}
