use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_string = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_string)?;
    Ok(config)
}

