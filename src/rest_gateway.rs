use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Command {
    command: String,
}

#[derive(Serialize)]
struct Response {
    response: String,
}

pub async fn run(addr: &str) {
    // Define a simple health check endpoint
    let health_route = warp::path!("health").map(|| warp::reply::json(&"OK"));

    // Define a command endpoint
    let command_route = warp::path!("command")
        .and(warp::post())
        .and(warp::body::json())
        .map(|cmd: Command| {
            println!("Received command: {:?}", cmd);

            // Modify the command in some way
            let response = Response {
                response: format!("Modified: {}", cmd.command),
            };

            warp::reply::json(&response)
        });

    // Combine routes
    let routes = health_route.or(command_route);

    // Parse the address
    let addr: std::net::SocketAddr = addr.parse().expect("Unable to parse socket address");

    // Print ready message
    println!("REST server is ready to receive commands on {}", addr);

    // Start the warp server
    warp::serve(routes).run(addr).await;
}
