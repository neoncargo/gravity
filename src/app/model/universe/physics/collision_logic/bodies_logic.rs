use super::Body;
use nannou::geom::Vec2;

pub struct BodiesLogic;

impl BodiesLogic {
    pub fn check_collision(&self, body1: &Body, body2: &Body) -> bool {
        unsafe {
            if check_collision(body1.clone(), body2.clone()) == 1 {
                true
            }
            else {
                false
            }
        }
    }

    pub fn calc_collision(&self, body1: &Body, body2: &Body) -> (Vec2, Vec2) {
        unsafe {
            let result = calc_collision(body1.clone(), body2.clone());
            (result.delta1, result.delta2)
        }
    }
}

#[repr(C)]
struct DeltaVelocities {
    delta1: Vec2,
    delta2: Vec2,
}

extern {
    fn check_collision(body1: Body , body2: Body) -> u8;

    fn calc_collision(body1: Body , body2: Body) -> DeltaVelocities;
}
