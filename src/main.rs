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
use tokio_tungstenite::tungstenite::protocol::Message;
use ws2mongo::config::Config;
use ws2mongo::mongodb::MongoClient;
use ws2mongo::websocket::WebSocketClient;

#[tokio::main]
async fn main() {
    env::set_var("DATABASE_NAME", "test");
    env::set_var("COLLECTION_NAME", "test");
    let config = Config::new().expect("Failed to load config");
    match config.print_as_json() {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing config: {}", e),
    }
    // Create messages as JSON objects
    let btc_subscribe = json!({
        "type": "subscribe",
        "symbol": "BTCUSD"
    });

    let eth_subscribe = json!({
        "type": "subscribe",
        "symbol": "ETHUSD"
    });

    // Convert JSON objects to string and wrap them as WebSocket messages
    let messages_to_send = vec![
        Message::text(btc_subscribe.to_string()),
        Message::text(eth_subscribe.to_string()),
    ];

    let mongoclient = MongoClient::new(config.clone())
        .await
        .expect("Failed to create MongoDB client");

    let mut wsclient = WebSocketClient::new(config, None, messages_to_send, mongoclient);
    wsclient.run().await;
}
