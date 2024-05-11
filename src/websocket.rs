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

use crate::config::Config;
use futures_util::{SinkExt, StreamExt}; // To access send and next methods
use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

pub struct WebSocketClient {
    pub config: Config,
    pub socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketClient {
    pub fn new(config: Config, socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Self {
        WebSocketClient { config, socket }
    }

    pub async fn connect(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let url = Url::parse(&config.websocket_url)?;

        let request = http::Request::builder()
            .uri(url.as_str())
            .header(
                "APCA-API-KEY-ID",
                &config.websocket_api_key.clone().unwrap_or_default(),
            )
            .header(
                "APCA-API-SECRET-KEY",
                &config.websocket_api_secret.clone().unwrap_or_default(),
            )
            .body(())?
            .into_client_request()?;

        let (ws_stream, _) = connect_async(request).await?;
        Ok(WebSocketClient::new(config, Some(ws_stream)))
    }

    // Asynchronously sends a message using the WebSocket
    pub async fn send_message(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        if let Some(socket) = &mut self.socket {
            socket
                .send(message)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        } else {
            Err("WebSocket connection not established".into())
        }
    }

    // Asynchronously receives a message from the WebSocket
    pub async fn receive_message(&mut self) -> Result<Message, Box<dyn Error>> {
        if let Some(socket) = &mut self.socket {
            match socket.next().await {
                Some(Ok(msg)) => Ok(msg),
                Some(Err(e)) => Err(Box::new(e) as Box<dyn Error>),
                None => Err("No message received".into()),
            }
        } else {
            Err("WebSocket connection not established".into())
        }
    }
}
