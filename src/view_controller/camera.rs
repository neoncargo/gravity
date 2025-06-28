use nannou::prelude::*;
use super::model::Model;
pub struct Camera {
    zoom: f32,
    position: Vec2,
}

impl Camera {
    pub fn new() -> Self { Self { zoom: 1., position: Vec2::new(0., 0.) } }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn view(&self, app: &App, frame: Frame, model: &Model) {
        let draw = app.draw();
        draw.background().color(BLACK);

        draw.text("Zoom: T/Y").y(frame.rect().y.end - 10.0).font_size(20);

        for body in model.bodies() {
            draw.ellipse()
            .color(WHITE)
            .radius(body.1 * self.zoom)
            .xy((body.0 - self.position) * self.zoom);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
