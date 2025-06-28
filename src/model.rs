mod universe;

use universe::Universe;
use universe::body::Body;
use nannou::geom::Vec2;

pub struct Model {
    universe: Universe,
}

impl Model {
    pub fn new() -> Self {
        let body1 = Body::new(Vec2::new(0.0, 0.0), 10.0, 1000.0, Vec2::new(0.0, 0.0));
        let body2 = Body::new(Vec2::new(500.0, 0.0), 10.0, 50.0, Vec2::new(0.0, 300.0));
        let body3 = Body::new(Vec2::new(-500.0, 0.0), 10.0, 100.0, Vec2::new(0.0, -300.0));

        let bodies = vec![body1, body2, body3];

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
}
