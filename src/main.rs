use std::env;
use std::fs;
use tokio::time::{sleep, Duration};
mod dispatcher;
mod web_socket_gateway;
mod rest_gateway;
mod charger_configs;

use web_socket_gateway::WebSocketGateway;
use dispatcher::handle_incoming_messages;
use tokio_tungstenite::tungstenite::protocol::Message;
use charger_configs::Config;

#[tokio::main]
async fn main() {
    // Parse the charger name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ChargerName>", args[0]);
        std::process::exit(1);
    }
    let charger_name = &args[1];

    // Read the configuration file
    let config_data = fs::read_to_string("charger_configs.json").expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("JSON was not well-formatted");

    // Find the charger configuration by name
    let charger_config = config.chargers.iter().find(|&c| c.name == *charger_name);
    let charger_config = match charger_config {
        Some(config) => config,
        None => {
            eprintln!("Charger {} not found in configuration", charger_name);
            std::process::exit(1);
        }
    };

    // Initialize the WebSocket server and channels
    let (gateway, mut incoming_rx, outgoing_tx) = WebSocketGateway::new();

    // Start the WebSocket gateway in its own async task
    let websocket_service_addr = charger_config.websocket_service_addr.clone();
    println!("Starting WebSocket server on: {}", websocket_service_addr);  // Debug print
    let gateway_task = tokio::spawn(async move {
        gateway.run(&websocket_service_addr).await;
    });

    // Start the REST server in its own async task
    let rest_service_addr = charger_config.rest_service_addr.clone();
    println!("Starting REST server on: {}", rest_service_addr);  // Debug print
    let _rest_task = tokio::spawn(async move {
        rest_gateway::run(&rest_service_addr).await;
    });

    // Process incoming messages (received from clients)
    tokio::spawn(async move {
        while let Some(message) = incoming_rx.recv().await {
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
