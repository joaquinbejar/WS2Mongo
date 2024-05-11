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
use std::error::Error;

#[cfg(test)]
mod websocket_tests {
    use super::*;
    use mockall::predicate::*;
    use tokio_tungstenite::tungstenite::error::Error as WsError;
    use tokio_tungstenite::tungstenite::Message as WsMessage;
    use tokio_tungstenite::tungstenite::{error::Error as WsError, Message as WsMessage};
    use ws2mongo::config::Config;
    use ws2mongo::websocket::WebSocketClient;

    trait MockWebSocketStream {
        fn new() -> Self;
        async fn send(&mut self, msg: WsMessage) -> Result<(), WsError>;
        async fn receive(&mut self) -> Option<Result<WsMessage, WsError>>;
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let mut mock_stream = MockWebSocketStream::new();
        let config = Config {
            websocket_url: String::from("ws://example.com"),
            websocket_api_key: None,
            websocket_api_secret: None,
            mongodb_uri: "".to_string(),
            database_name: "".to_string(),
            collection_name: "".to_string(),
            mongodb_user: None,
            mongodb_password: None,
            mongodb_auth_source: None,
            mongodb_auth_mechanism: None,
        };

        mock_stream
            .expect_send()
            .times(1)
            .with(eq(WsMessage::Text("Hello WebSocket".to_string())))
            .returning(|_| Ok(()));

        let mut client = WebSocketClient::new(config, Some(mock_stream));

        let result = client
            .send_message(WsMessage::Text("Hello WebSocket".to_string()))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_receive_message_success() {
        let mut mock_stream = MockWebSocketStream::new();
        let config = Config {
            websocket_url: String::from("ws://example.com"),
            websocket_api_key: None,
            websocket_api_secret: None,
            mongodb_uri: "".to_string(),
            database_name: "".to_string(),
            collection_name: "".to_string(),
            mongodb_user: None,
            mongodb_password: None,
            mongodb_auth_source: None,
            mongodb_auth_mechanism: None,
        };

        let expected_msg = WsMessage::Text("Hello from WebSocket".to_string());
        mock_stream
            .expect_receive()
            .times(1)
            .returning(move || Some(Ok(expected_msg.clone())));

        let mut client = WebSocketClient::new(config, Some(mock_stream));

        let result = client.receive_message().await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            WsMessage::Text("Hello from WebSocket".to_string())
        );
    }
}
