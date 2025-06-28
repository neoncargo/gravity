mod universe;
mod config_manager;
mod save_manager;

use std::fs;
use nannou::geom::Vec2;

use universe::Universe;
use universe::body::Body;

use config_manager::ConfigManager;
use save_manager::SaveManager;

use once_cell::sync::Lazy;

static CONFIG_MANAGER: Lazy<ConfigManager> = Lazy::new(|| { ConfigManager::new_with_load() });

pub struct Model {
    universe: Universe,
    _save_manager: SaveManager,
}

impl Model {
    pub fn new() -> Self {
        let config_manager = ConfigManager::new_with_load();
        let save_manager = SaveManager;

        let save = save_manager.load_save_from_file(config_manager.save_file_name());

        Self {
            universe: Universe::new(save.bodies),
            _save_manager: save_manager,
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
