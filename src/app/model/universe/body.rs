use nannou::geom::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Body {
    position: Vec2,
    radius: f32,
    mass: f32,
    velocity: Vec2,
}

impl Body {
    pub fn _new(position: Vec2, radius: f32, mass: f32, velocity: Vec2) -> Self {
        Self { position, radius, mass, velocity }
    }

    pub fn position(&self) -> Vec2 { self.position }

    pub fn set_position(&mut self, position: Vec2) { self.position = position; }

    pub fn radius(&self) -> f32 { self.radius }

    pub fn mass(&self) -> f32 { self.mass }

    pub fn velocity(&self) -> Vec2 { self.velocity }

    pub fn set_velocity(&mut self, velocity: Vec2) { self.velocity = velocity; }

    pub fn _set_velocity_x(&mut self, x: f32) { self.velocity.x = x; }

    pub fn _set_velocity_y(&mut self, y: f32) { self.velocity.y = y; }
}
