mod bodies_logic;

use bodies_logic::BodiesLogic;
use nannou::geom::Vec2;
use super::Body;

pub struct CollisionLogic {
    bodies_logic: BodiesLogic,
}

impl CollisionLogic {
    pub fn new() -> Self { Self { bodies_logic: BodiesLogic } }

    pub fn check_collision(&self, body1: &Body, body2: &Body) -> bool {
        self.bodies_logic.check_collision(body1, body2)
    }

    pub fn calc_collision(&self, body1: &Body, body2: &Body) -> (Vec2, Vec2) {
        self.bodies_logic.calc_collision(body1, body2)
    }
}
