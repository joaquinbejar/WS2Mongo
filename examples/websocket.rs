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

use ws2mongo::config::{Config};
use ws2mongo::websocket::{WebSocketClient};
use tokio_tungstenite::tungstenite::protocol::Message;


#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load config");
    let mut client = WebSocketClient::new(config, None).await.expect("Failed to create WebSocket client");

    client.connect().await.expect("Failed to connect to WebSocket");

    // Ejemplo de cómo enviar un mensaje
    client.send_message(Message::Text("Hello WebSocket".to_string())).await.expect("Failed to send message");

    // Ejemplo de cómo recibir un mensaje
    let msg = client.receive_message().await.expect("Failed to receive message");
    println!("Received: {:?}", msg);
}
