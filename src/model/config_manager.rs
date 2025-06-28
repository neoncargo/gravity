mod config;

use config::Config;

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn _new() -> Self {
        Self { config: Config::new() }
    }

    pub fn new_with_load() -> Self {
        Self { config: Config::from_file(None) }
    }

    pub fn is_gravity_enabled(&self) -> bool {
        self.config.is_gravity_enabled()
    }

    pub fn save_file_name(&self) -> &str {
        self.config.save_file_name()
    }
}
