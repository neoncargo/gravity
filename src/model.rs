mod universe;

use std::fs;
use log::{warn, debug};
use nannou::geom::Vec2;

use universe::Universe;
use universe::body::Body;

pub struct Model {
    universe: Universe,
}

impl Model {
    pub fn new() -> Self {
        let save = fs::read_to_string("save.json").unwrap_or(String::new());

        let bodies: Vec<Body> = serde_json::from_str(&save).unwrap_or_else(|e| {
            warn!("Failed to read save: {}", e);
            Vec::new()
        });

        debug!("Loaded bodies: {:#?}", bodies);

        Self {
            universe: Universe::new(bodies)
        }
    }

    pub fn bodies(&self) -> Vec<(Vec2, f32)> {
        self.universe.bodies()
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        self.universe.update(delta_time_sec);
    }

    pub fn save(&self) {
        let str = serde_json::to_string_pretty(&self.universe.real_bodies()).expect("JSON TO STRING FAILED");
        fs::write("save.json", str).expect("FS FAILED");
    }
}
