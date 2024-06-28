use serde::Deserialize;
use std::{error::Error, fs};


#[derive(Deserialize, Debug, Clone)]
pub struct ChargerConfig {
    pub name: String,
    pub rest_service_addr: String,
    pub websocket_service_addr: String,
    // pub redis_connection: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub chargers: Vec<ChargerConfig>,
}

pub fn load_charger_config(charger_name: &str) -> Result<ChargerConfig, Box<dyn Error>> {
    // Read the configuration file
    let config_data = fs::read_to_string("charger_configs.json")?;
    let config: Config = serde_json::from_str(&config_data)?;

    // Find the charger configuration by name
    let charger_config = config.chargers.iter().find(|&c| c.name == charger_name);
    match charger_config {
        Some(config) => Ok(config.clone()),
        None => Err(format!("Charger {} not found in configuration", charger_name).into()),
    }
}

