use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ChargerConfig {
    pub name: String,
    pub rest_service_addr: String,
    pub websocket_service_addr: String,
    pub redis_connection: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub chargers: Vec<ChargerConfig>,
}
