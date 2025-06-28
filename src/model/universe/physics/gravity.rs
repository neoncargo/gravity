pub struct Gravity;

use super::Body;
use nannou::geom::Vec2;

impl Gravity {
    pub fn delta_velocities(&self, body1: &Body, body2: &Body, delta_time_sec: f32) -> (Vec2, Vec2) {
        let diff = body2.position() - body1.position();

        let force = (body1.mass() * body2.mass()) / (diff.length().powi(2)) * 100_000.0;

        let delta_speed1 = force / body1.mass() * delta_time_sec;
        let delta_speed2 = force / body2.mass() * delta_time_sec;

        let delta1 = delta_speed1 * diff.normalize_or_zero();
        let delta2 = delta_speed2 * (-diff.normalize_or_zero());

        (delta1, delta2)
    }
}
