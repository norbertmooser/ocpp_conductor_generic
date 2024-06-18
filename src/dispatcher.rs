// src/dispatcher.rs

use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn handle_incoming_messages(mut incoming_rx: Receiver<Message>) {
    while let Some(message) = incoming_rx.recv().await {
        match message {
            Message::Text(text) => {
                println!("Received Text: {}", text);
                // Handle incoming text messages here
            }
            Message::Binary(bin) => {
                println!("Received Binary: {:?}", bin);
                // Handle incoming binary messages here
            }
            Message::Ping(ping) => {
                println!("Received Ping: {:?}", ping);
                // Handle ping messages here
            }
            Message::Pong(pong) => {
                println!("Received Pong: {:?}", pong);
                // Handle pong messages here
            }
            Message::Close(close) => {
                println!("Received Close: {:?}", close);
                // Handle close messages here
            }
            Message::Frame(_) => todo!(),
        }
    }
}
