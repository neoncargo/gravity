pub mod body;

mod physics;

use body::Body;
use physics::Physics;
use nannou::geom::Vec2;

pub struct Universe {
    bodies: Vec<Body>,
    physics: Physics,
}

impl Universe {
    pub fn new(bodies: Vec<Body>) -> Self { Self { bodies, physics: Physics::new() } }

    pub fn bodies(&self) -> Vec<(Vec2, f32)> {
        let mut bodies = Vec::new();
        for body in &self.bodies {
            bodies.push((body.position(), body.radius()));
        }

        bodies
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        self.physics.update(&mut self.bodies, delta_time_sec);
    }
}
