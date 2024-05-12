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

use std::error::Error;
use tokio_tungstenite::tungstenite::protocol::Message;
use serde_json::Value;

pub fn pretty_print(message: Message) -> Result<(), Box<dyn Error>> {
    match message {
        Message::Text(text) => {
            let parsed_json: Value = serde_json::from_str(&text)?;
            println!("Text: {}", serde_json::to_string_pretty(&parsed_json)?);
        },
        Message::Binary(data) => {
            let parsed_json: Value = serde_json::from_slice(&data)?;
            println!("Binary: {}", serde_json::to_string_pretty(&parsed_json)?);
        },
        Message::Ping(ping_data) => {
            println!("Ping: {:?}", ping_data);
        },
        Message::Pong(pong_data) => {
            println!("Pong: {:?}", pong_data);
        },
        Message::Close(close_frame) => {
            if let Some(frame) = close_frame {
                println!("Close: code={}, reason={}", frame.code, frame.reason);
            } else {
                println!("Close: no details");
            }
        },
        _ => {
            eprintln!("Received an unknown message type: {:?}", message);
        },
    }
    Ok(())
}