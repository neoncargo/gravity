const DEFAULT_CONFIG_NAME: &str = "config.json";

use std::fs;

use log::{debug, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    is_gravity_enabled: bool,
    save_file_name: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            is_gravity_enabled: false,
            save_file_name: String::new(),
        }
    }

    pub fn from_file(name: Option<&str>) -> Config {
        let config_name = name.unwrap_or(DEFAULT_CONFIG_NAME);
        debug!("Config name: {config_name}");

        let config_str = fs::read_to_string(config_name).unwrap_or_else(|e| {
            warn!("Failed to read config: {}", e);
            String::new()
        });
        debug!("Config str: {config_str}");

        serde_json::from_str(&config_str).unwrap_or_else(|e| {
            warn!("Failed to deserialize config: {}", e);
            Config::new()
        })
    }

    pub fn is_gravity_enabled(&self) -> bool {
        self.is_gravity_enabled
    }

    pub fn save_file_name(&self) -> &str {
        self.save_file_name.as_ref()
    }
}
