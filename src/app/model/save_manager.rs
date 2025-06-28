mod save;

use super::Body;
pub use save::Save;

pub struct SaveManager;

impl SaveManager {
    pub fn load_save_from_file(&self, name: &str) -> Save {
        Save::from_file(name)
    }
}
