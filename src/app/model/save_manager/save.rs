use std::fs;
use log::{trace, debug, warn};
use serde::{Serialize, Deserialize};

use super::Body;

#[derive(Serialize, Deserialize, Debug)]
pub struct Save {
    pub bodies: Vec<Body>,
}

impl Save {
    pub fn new() -> Self { Self { bodies: Vec::new() } }

    pub fn from_file(name: &str) -> Save {
        debug!("Save name: {name}");

        let save_str = fs::read_to_string(name).unwrap_or_else(|e| {
            warn!("Failed to read save: {}", e);
            String::new()
        });
        trace!("Save str: {save_str}");

        serde_json::from_str(&save_str).unwrap_or_else(|e| {
            warn!("Failed to deserialize save: {}", e);
            Save::new()
        })
    }


}
