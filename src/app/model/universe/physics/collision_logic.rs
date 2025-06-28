mod bodies_logic;

use bodies_logic::BodiesLogic;

pub struct CollisionLogic {
    _bodies_logic: BodiesLogic,
}

impl CollisionLogic {
    pub fn new() -> Self { Self { _bodies_logic: BodiesLogic } }
}
