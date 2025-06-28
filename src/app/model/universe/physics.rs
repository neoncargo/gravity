mod gravity;
mod collision_logic;

use log::debug;
use nannou::geom::Vec2;

use gravity::Gravity;
use collision_logic::CollisionLogic;
use crate::app::model::CONFIG_MANAGER;
use super::Body;


pub struct Physics {
    gravity: Gravity,
    collision_logic: CollisionLogic,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: Gravity,
            collision_logic: CollisionLogic::new(),
        }
    }

    pub fn update(&self, bodies: &mut Vec<Body>, delta_time_sec: f32) {
        let mut delta_vels: Vec<Vec2> = vec![Vec2::ZERO ; bodies.len()];

        if CONFIG_MANAGER.is_gravity_enabled() {
            for i in 0..bodies.len() {
                for j in i+1..bodies.len() {
                    let (delta1, delta2) = self.gravity.delta_velocities(&bodies[i], &bodies[j], delta_time_sec);
                    delta_vels[i] += delta1;
                    delta_vels[j] += delta2;
                }
            }
        }

        for i in 0..bodies.len() {
            for j in i+1..bodies.len() {
                if !self.collision_logic.check_collision(&bodies[i], &bodies[j]) {
                    continue;
                }
                debug!("collision detected");

                let (delta1, delta2) = self.collision_logic.calc_collision(&bodies[i], &bodies[j]);
                delta_vels[i] += delta1;
                delta_vels[j] += delta2;
            }
        }

        for i in 0..bodies.len() {
            let new_vel = bodies[i].velocity() + delta_vels[i];
            bodies[i].set_velocity(new_vel);
        }

        update_positions(bodies, delta_time_sec);
    }
}

fn update_positions(bodies: &mut Vec<Body>, delta_time_sec: f32) {
    for body in bodies {
        let new_position = body.position() + body.velocity() * delta_time_sec;
        body.set_position(new_position);
    }
}
