mod gravity;
mod collision_logic;

use nannou::geom::Vec2;
use threadpool::ThreadPool;

use gravity::Gravity;
use collision_logic::CollisionLogic;
use crate::app::model::CONFIG_MANAGER;
use super::Body;


pub struct Physics {
    gravity: Gravity,
    _collision_logic: CollisionLogic,
    thread_pool: threadpool::ThreadPool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: Gravity,
            _collision_logic: CollisionLogic::new(),
            thread_pool: ThreadPool::new(4),
        }
    }

    pub fn update(&self, bodies: &mut Vec<Body>, delta_time_sec: f32) {
        let mut delta_vels: Vec<Vec2> = vec![Vec2::ZERO ; bodies.len()];

        if CONFIG_MANAGER.is_gravity_enabled() {
            let (tx, rx) = std::sync::mpsc::channel();

            for i in 0..bodies.len() {
                for j in i+1..bodies.len() {
                    let tx = tx.clone();
                    let body1 = bodies[i].clone();
                    let body2 = bodies[j].clone();
                    self.thread_pool.execute(move || {
                        let gravity = Gravity;
                        tx.send((gravity.delta_velocities(&body1, &body2, delta_time_sec), i, j)).expect("send should be ok");
                    });
                }
            }
            drop(tx);

            for received in rx {
                delta_vels[received.1] += received.0.0;
                delta_vels[received.2] += received.0.1;
            }

            self.thread_pool.join();
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
