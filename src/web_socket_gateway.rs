
//src/web_socket_gateway.rs

///The WebSocketGateway is solely responsible for setting up the WebSocket server, accepting connections, 
///and managing the basic message flows (incoming and outgoing) without processing them.

use tokio::sync::{mpsc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};

/// The variable incoming_tx in the context of the WebSocketGateway code you provided is named to indicate its role and functionality:
///
/// # incoming: This prefix suggests that the variable is related to data or messages that are coming into the system. In this case, 
///   it refers to messages received from WebSocket clients that are connected to the server.
/// # tx: This is an abbreviation for "transmitter" or "transmit". In asynchronous programming, particularly when using channels, 
///   tx is commonly used to denote the sending end of a channel. This part of the channel is responsible for transmitting data to 
///   another part of the program, typically a corresponding receiver (rx).
pub struct WebSocketGateway {
    incoming_tx: mpsc::Sender<Message>,
    outgoing_rx: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl WebSocketGateway {
    pub fn new() -> (Self, mpsc::Receiver<Message>, mpsc::Sender<Message>) {
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
        let listener = TcpListener::bind(addr).await.expect("Can't bind to address");
        println!("Listening on: {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let incoming_tx = self.incoming_tx.clone();
            let outgoing_rx = self.outgoing_rx.clone();
            
            tokio::spawn(async move {
                let ws_stream = tokio_tungstenite::accept_async(stream)
                    .await
                    .expect("Error during the websocket handshake");
                let (mut write, mut read) = ws_stream.split();

                // Read messages from WebSocket and forward them
                while let Some(message) = read.next().await {
                    if let Ok(msg) = message {
                        if incoming_tx.send(msg).await.is_err() {
                            eprintln!("Error sending message to the channel");
                            break;
                        }
                    }
                }

                // Read messages from outgoing_rx and send to WebSocket
                while let Some(message) = outgoing_rx.lock().await.recv().await {
                    if let Err(e) = write.send(message).await {
                        eprintln!("Error sending message to WebSocket: {}", e);
                        break;
                    }
                }
            });
        }
    }
}
