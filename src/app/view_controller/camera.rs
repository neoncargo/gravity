use super::Vec2;

pub struct Camera {
    zoom: f32,
    position: Vec2,
}

impl Camera {
    pub fn new() -> Self { Self { zoom: 1., position: Vec2::new(0., 0.) } }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn world_to_camera(&self, body: (Vec2, f32)) -> (Vec2, f32) {
        let new_pos = (body.0 - self.position()) * self.zoom();
        let new_size = body.1 * self.zoom();

        (new_pos, new_size)
    }
}
