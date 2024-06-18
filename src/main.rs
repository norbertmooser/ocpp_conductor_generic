// src/main.rs

use tokio::time::{sleep, Duration};
mod web_socket_gateway;
mod dispatcher;

use web_socket_gateway::WebSocketGateway;
use dispatcher::handle_incoming_messages;

#[tokio::main]
async fn main() {
    let (gateway, incoming_rx, _outgoing_tx) = WebSocketGateway::new();

    // Spawn the WebSocket gateway task
    let gateway_task = tokio::spawn(async move {
        gateway.run("127.0.0.1:8080").await;
    });

    // Spawn the task to handle incoming WebSocket messages
    tokio::spawn(handle_incoming_messages(incoming_rx));

    // Spawn a task to run a counter concurrently
    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            println!("Counter: {}", counter);
            counter += 1;
            sleep(Duration::from_secs(1)).await;
        }
    });

    // Await the gateway task to ensure it runs indefinitely
    gateway_task.await.expect("WebSocket server failed");
}
