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
use mongodb::bson::Document;
use mongodb::options::{AuthMechanism, ClientOptions};
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

use mongodb::{
    bson::doc, error::Error as MongoError, error::Result as MongoResult, Client, Collection,
};

pub async fn test_mongo_connection(client: &Client, db: &str) -> MongoResult<()> {
    let database = client.database(db);
    let command = doc! {"ping": 1};
    let result = database.run_command(command, None).await?;

    // Verifica que la respuesta contenga 'ok: 1.0'
    match result.get_f64("ok") {
        Ok(ok) if ok == 1.0 => {
            println!("Successfully connected to MongoDB.");
            Ok(())
        }
        Ok(_) => {
            println!("Received an unexpected response from MongoDB.");
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected response to ping",
            )))
        }
        Err(e) => {
            println!("Failed to retrieve 'ok' from response: {}", e);
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}", e),
            )))
        }
    }
}

pub struct MongoClient {
    collection: Collection<Document>,
    sender: Sender<Value>,
    receiver: Arc<Mutex<Receiver<Value>>>,
}

impl MongoClient {
    pub async fn new(config: Config) -> Result<Arc<Self>, Box<dyn Error>> {
        // Parse MongoDB URI and create client options
        let mut client_options = ClientOptions::parse(&config.mongodb_uri).await?;
        let auth_source_str: &str = config.mongodb_auth_source.as_deref().unwrap_or("admin");

        // Optionally set the username, password, auth source, and mechanism if provided
        if let Some(user) = config.mongodb_user {
            let mut credential = mongodb::options::Credential::default();
            credential.username = Some(user);
            credential.password = config.mongodb_password;
            credential.source = Some(auth_source_str.to_string());
            if let Some(mech) = &config.mongodb_auth_mechanism {
                credential.mechanism = match mech.as_str() {
                    "SCRAM-SHA-1" => Some(AuthMechanism::ScramSha1),
                    "SCRAM-SHA-256" => Some(AuthMechanism::ScramSha256),
                    "MONGODB-CR" => Some(AuthMechanism::MongoDbCr),
                    // "MONGODB_AWS" => Some(AuthMechanism::MongoDbAws),
                    "MONGODB-X509" => Some(AuthMechanism::MongoDbX509),
                    "PLAIN" => Some(AuthMechanism::Plain),
                    mechanism => {
                        return Err(format!("Unsupported auth mechanism: {}", mechanism).into())
                    }
                };
            }

            client_options.credential = Some(credential);
        }

        let client = Client::with_options(client_options)?;

        // Test the connection to MongoDB
        if let Err(_e) = test_mongo_connection(&client, auth_source_str).await {
            return Err("Error connecting to MongoDB".into());
        }

        let db = client.database(&config.database_name);
        let collection = db.collection(&config.collection_name);

        let (sender, receiver) = mpsc::channel(100); // Buffer size of 100

        let instance = Arc::new(MongoClient {
            collection,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        });

        // Use `tokio::spawn` to create a new independent async task
        let instance_clone = Arc::clone(&instance);
        tokio::spawn(async move {
            instance_clone.start().await;
        });

        Ok(instance)
    }

    pub async fn start(&self) {
        // Clone the receiver
        let receiver = Arc::clone(&self.receiver);

        // Lock the receiver
        let mut receiver = receiver.lock().await;

        while let Some(json_value) = receiver.recv().await {
            // // Convert `json_value` to a `Document` here
            // let document = match serde_json::from_value::<Document>(json_value) {
            //     Ok(document) => document,
            //     Err(e) => {
            //         eprintln!("Error converting JSON to a document: {}", e);
            //         continue;
            //     }
            // };
            //
            // // Insert the document into MongoDB
            // if let Err(e) = self.collection.insert_one(document, None).await {
            //     eprintln!("Error inserting document into MongoDB: {}", e);
            // }
            match json_value {
                Value::Object(_) => {
                    // Directly try to convert the Value to a Document
                    let document = match serde_json::from_value::<Document>(json_value) {
                        Ok(document) => document,
                        Err(e) => {
                            eprintln!("Error converting JSON to a document: {}", e);
                            continue;
                        }
                    };

                    // Insert the document into MongoDB
                    if let Err(e) = self.collection.insert_one(document, None).await {
                        eprintln!("Error inserting document into MongoDB: {}", e);
                    }
                },
                Value::Array(array) => {
                    // Iterate over each item in the array, assuming each item is an object
                    for item in array {
                        let document = match serde_json::from_value::<Document>(item) {
                            Ok(document) => document,
                            Err(e) => {
                                eprintln!("Error converting JSON item to a document: {}", e);
                                continue;
                            }
                        };

                        // Insert each document into MongoDB
                        if let Err(e) = self.collection.insert_one(document, None).await {
                            eprintln!("Error inserting document into MongoDB: {}", e);
                        }
                    }
                },
                _ => eprintln!("Received JSON is neither an object nor an array"),
            }
        }

    }

    pub async fn enqueue(&self, message: Message) -> Result<(), Box<dyn Error>> {
        match message {
            Message::Text(text) => {
                match serde_json::from_str::<Value>(&text) {
                    Ok(json) => {
                        // if the JSON is successfully parsed, send it to the sender
                        self.sender.send(json).await.map_err(|e| Box::new(e) as _)
                    }
                    Err(_) => {
                        // if the JSON is not successfully parsed, continue.
                        Ok(())
                    }
                }
            }
            Message::Binary(data) => {
                match serde_json::from_slice::<Value>(&data) {
                    Ok(json) => {
                        // if the JSON is successfully parsed, send it to the sender
                        self.sender.send(json).await.map_err(|e| Box::new(e) as _)
                    }
                    Err(_) => {
                        // if the JSON is not successfully parsed, continue.
                        Ok(())
                    }
                }
            }

            Message::Ping(ping_data) => {
                println!("Ping: {:?}", ping_data);
                Ok(())
            }
            Message::Pong(pong_data) => {
                println!("Pong: {:?}", pong_data);
                Ok(())
            }
            Message::Close(close_frame) => {
                if let Some(frame) = close_frame {
                    println!("Close: code={}, reason={}", frame.code, frame.reason);
                } else {
                    println!("Close: no details");
                }
                Ok(())
            }
            _ => Err("Unsupported message format".into()),
        }
    }
}
