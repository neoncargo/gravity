mod model;
mod view_controller;

use model::Model;
use view_controller::ViewController;
use nannou::geom::Vec2;

pub use view_controller::Camera;

pub struct App {
    model: Model,
    view_controller: ViewController,
}

impl App {
    pub fn new() -> Self {
         Self { model: model::Model::new(), view_controller: ViewController::new() }
    }

    pub fn bodies(&self) -> Vec<(Vec2, f32)> {
        self.model.bodies()
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        self.model.update(delta_time_sec)
    }

    pub fn world_to_camera(&self, body: (Vec2, f32)) -> (Vec2, f32) {
        self.view_controller.world_to_camera(body)
    }

    pub fn camera(&mut self) -> &mut Camera {
        &mut self.view_controller.camera
    }
}
