use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub server_address: String,
    pub ports: Vec<u16>,
    pub default_error_pages: HashMap<String, String>,
    pub max_body_size: usize,
    pub routes: Vec<Route>,
    // Add other configuration fields as necessary
}

#[derive(Deserialize)]
pub struct Route {
    pub path: String,
    pub methods: Vec<String>,
    // Add other route-specific fields
}

impl ServerConfig {
    // Function to create a ServerConfig from a file path
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the configuration file to a String
        let config_string = fs::read_to_string(path)?;
        // Parse the String using the `toml` crate
        let config: ServerConfig = toml::from_str(&config_string)?;
        Ok(config)
    }
}

// Add other necessary structs and enums that match your TOML structure
