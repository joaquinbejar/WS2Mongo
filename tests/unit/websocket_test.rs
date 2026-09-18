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

use futures_util::{SinkExt, StreamExt};

#[cfg(test)]
mod websocket_tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio_tungstenite::tungstenite::Message as WsMessage;
    use ws2mongo::config::Config;
    use ws2mongo::mongodb::MongoClient;
    use ws2mongo::websocket::WebSocketClient;

    /// Spawns a local WebSocket echo server and returns its address.
    async fn spawn_echo_server() -> std::net::SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();
            while let Some(Ok(msg)) = ws.next().await {
                if msg.is_text() || msg.is_binary() {
                    ws.send(msg).await.unwrap();
                }
            }
        });
        addr
    }

    fn test_config(addr: std::net::SocketAddr) -> Config {
        Config {
            websocket_url: format!("ws://{addr}"),
            websocket_api_key: None,
            websocket_api_secret: None,
            mongodb_uri: "mongodb://localhost:27017".to_string(),
            database_name: "test".to_string(),
            collection_name: "test".to_string(),
            mongodb_user: None,
            mongodb_password: None,
            mongodb_auth_source: "admin".to_string(),
            mongodb_auth_mechanism: "SCRAM-SHA-256".to_string(),
        }
    }

    #[tokio::test]
    #[ignore = "requires a live MongoDB at mongodb://localhost:27017 to build MongoClient"]
    async fn test_send_and_receive_message_roundtrip() {
        let addr = spawn_echo_server().await;
        let config = test_config(addr);
        let mongo_client = MongoClient::new(config.clone())
            .await
            .expect("Failed to create MongoDB client");

        let mut client = WebSocketClient::new(config, None, vec![], mongo_client);
        client
            .connect()
            .await
            .expect("Failed to connect to echo server");

        let sent = WsMessage::text("Hello WebSocket");
        client
            .send_message(sent.clone())
            .await
            .expect("Failed to send message");

        let received = client
            .receive_message()
            .await
            .expect("Failed to receive message");
        assert_eq!(received, sent);
    }
}
