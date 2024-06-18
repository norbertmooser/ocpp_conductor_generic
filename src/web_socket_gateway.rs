// src/web_socket_gateway.rs

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct WebSocketGateway {
    incoming_tx: Sender<Message>,
    outgoing_rx: Arc<Mutex<Receiver<Message>>>,
}

impl WebSocketGateway {
    pub fn new() -> (Self, Receiver<Message>, Sender<Message>) {
        let (incoming_tx, incoming_rx) = mpsc::channel(100);
        let (outgoing_tx, outgoing_rx) = mpsc::channel(100);

        (
            Self {
                incoming_tx,
                outgoing_rx: Arc::new(Mutex::new(outgoing_rx)),
            },
            incoming_rx,
            outgoing_tx,
        )
    }

    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Can't bind to address");

        println!("Listening on: {}", addr);

        let incoming_tx = self.incoming_tx.clone();
        let outgoing_rx = Arc::clone(&self.outgoing_rx);

        while let Ok((stream, _)) = listener.accept().await {
            let incoming_tx = incoming_tx.clone();
            let outgoing_rx = Arc::clone(&outgoing_rx);
            tokio::spawn(async move {
                let ws_stream = accept_async(stream)
                    .await
                    .expect("Error during the websocket handshake occurred");

                let (mut write, mut read) = ws_stream.split();

                let incoming_tx_clone = incoming_tx.clone();
                let outgoing_rx_clone = Arc::clone(&outgoing_rx);

                tokio::spawn(async move {
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                if let Err(e) = incoming_tx_clone.send(msg).await {
                                    eprintln!("Failed to send incoming message: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error receiving message: {}", e);
                                break;
                            }
                        }
                    }
                });

                tokio::spawn(async move {
                    while let Some(msg) = outgoing_rx_clone.lock().await.recv().await {
                        if let Err(e) = write.send(msg).await {
                            eprintln!("Failed to send outgoing message: {}", e);
                            break;
                        }
                    }
                });
            });
        }
    }
}
