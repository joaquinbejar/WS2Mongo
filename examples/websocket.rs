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
use std::error::Error;
use tokio_tungstenite::tungstenite::protocol::Message;
use ws2mongo::config::Config;
use ws2mongo::websocket::WebSocketClient;
use serde_json::Value;

fn pretty_print(message: Message) -> Result<(), Box<dyn Error>> {
    match message {
        Message::Text(text) => {
            let parsed_json: Value = serde_json::from_str(&text)?;
            println!("{}", serde_json::to_string_pretty(&parsed_json)?);
        },
        Message::Binary(data) => {
            // Assuming binary data might also be JSON. Adjust as necessary.
            let parsed_json: Value = serde_json::from_slice(&data)?;
            println!("{}", serde_json::to_string_pretty(&parsed_json)?);
        },
        _ => eprintln!("Received a message that's neither text nor binary."),
    }
    Ok(())
}
#[tokio::main]
async fn main() {
    env::set_var("WEBSOCKET_URL", "ws://localhost:5678");
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    env::set_var("DATABASE_NAME", "test");
    env::set_var("COLLECTION_NAME", "test");
    let config = Config::new().expect("Failed to load config");
    let mut client = WebSocketClient::new(config, None);

    // Run the client with the message processing function
    client.run(pretty_print).await;
}
