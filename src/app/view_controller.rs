mod camera;

pub use camera::Camera;
use super::Vec2;

pub struct ViewController {
    pub camera: Camera,
}

impl ViewController {
    pub fn new() -> Self { Self { camera: Camera::new() } }

    pub fn world_to_camera(&self, body: (Vec2, f32)) -> (Vec2, f32) {
        self.camera.world_to_camera(body)
    }
}
