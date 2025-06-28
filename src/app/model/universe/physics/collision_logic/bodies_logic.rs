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

    pub fn calc_collision_positions(&self, body1: &Body, body2: &Body) -> (Vec2, Vec2) {
        let radius_sum = body1.radius() + body2.radius();

        let distance_between_centers = Vec2::distance(body1.position(), body2.position());

        let delta = radius_sum - distance_between_centers;

        let delta_to_each = delta / 2.0;

        let position1 = body1.position() + (body1.position() - body2.position()).normalize_or_zero() * delta_to_each;
        let position2 = body2.position() + (body2.position() - body1.position()).normalize_or_zero() * delta_to_each;

        (position1, position2)
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
