use tokio::time::{sleep, Duration};
mod dispatcher;
mod web_socket_gateway;

use web_socket_gateway::WebSocketGateway;
use dispatcher::handle_incoming_messages;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    // Initialize the WebSocket server and channels
    let (gateway, mut incoming_rx, outgoing_tx) = WebSocketGateway::new();

    // Start the WebSocket gateway in its own async task
    let gateway_task = tokio::spawn(async move {
        gateway.run("127.0.0.1:8080").await;
    });

    // Process incoming messages (received from clients)
    tokio::spawn(async move {
        while let Some(message) = incoming_rx.recv().await {
            // Assuming the function `handle_incoming_messages` now takes Message directly
            handle_incoming_messages(message).await;
        }
    });

    // Optionally, send messages periodically to all clients
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            if let Err(e) = outgoing_tx.send(Message::Text("Hello from server!".to_string())).await {
                eprintln!("Failed to send message: {}", e);
                break;
            }
        }
    });

    // Spawn a task to run a counter concurrently
    tokio::spawn(async move {
        let mut counter: i32 = 0;
        loop {
            println!("Counter: {}", counter);
            counter += 1;
            sleep(Duration::from_secs(1)).await;
        }
    });

    // Await the gateway task to ensure it runs for the lifetime of the application
    if let Err(e) = gateway_task.await {
        eprintln!("WebSocket server task failed: {}", e);
    }
}
