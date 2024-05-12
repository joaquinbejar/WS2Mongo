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
use tokio_tungstenite::{
    connect_async, connect_async_tls_with_config, tungstenite::protocol::Message, Connector,
    MaybeTlsStream, WebSocketStream,
};
use tungstenite::client::IntoClientRequest;
use url::Url;

pub struct WebSocketClient {
    pub config: Config,
    pub socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    initial_messages: Vec<Message>, // Store initial messages to be sent upon connection
}

impl WebSocketClient {
    pub fn new(
        config: Config,
        socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        initial_messages: Vec<Message>,
    ) -> Self {
        WebSocketClient {
            config,
            socket,
            initial_messages, // Initialize with the provided messages
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let url = Url::parse(&self.config.websocket_url)?;
        let url = Url::parse(&self.config.websocket_url).unwrap();
        // print the url
        // println!("url: {:?}", url);

        let mut request_builder = http::Request::builder()
            .uri(url.as_str())
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .header("Sec-WebSocket-Version", "13")
            .header("host", url.host_str().unwrap())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket");

        // Only add the headers if they are set
        if let Some(api_key) = &self.config.websocket_api_key {
            request_builder = request_builder.header("APCA-API-KEY-ID", api_key);
        }

        if let Some(api_secret) = &self.config.websocket_api_secret {
            request_builder = request_builder.header("APCA-API-SECRET-KEY", api_secret);
        }

        let request = request_builder
            .body(())
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .into_client_request()?;

        if url.scheme() == "wss" {
            let tls_connector = native_tls::TlsConnector::builder().build().unwrap();
            let connector = Connector::NativeTls(tls_connector);
            let (ws_stream, _) =
                connect_async_tls_with_config(request, None, false, Some(connector)).await?;
            self.socket = Some(ws_stream);
        } else {
            let (ws_stream, _) = connect_async(request).await?;
            self.socket = Some(ws_stream);
        }

        // let (ws_stream, _) = connect_async(request).await?;
        // self.socket = Some(ws_stream);

        // Send initial messages if the connection is successful
        if let Some(ref mut socket) = self.socket {
            for message in &self.initial_messages {
                socket
                    .send(message.clone())
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            }
        }

        Ok(())
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

    // Manages the WebSocket connection
    pub async fn run<F>(&mut self, process_message: F)
    where
        F: Fn(Message) -> Result<(), Box<dyn Error>> + Copy,
    {
        loop {
            let maybe_socket = self.socket.take(); // Temporarily take the socket

            if let Some(socket) = maybe_socket {
                let (_write, mut read) = socket.split();
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(message) => {
                            if let Err(e) = process_message(message) {
                                eprintln!("Error processing message: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error in receiving message: {}", e);
                            break; // Exit the inner loop to attempt reconnection
                        }
                    }
                }
            }

            // Attempt to reconnect
            if let Err(e) = self.connect().await {
                eprintln!("Failed to reconnect: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Delay before retrying
            }
        }
    }
}
