mod view_controller;
mod model;

fn main() {
    nannou::app(view_controller::nannou_model)
    .event(view_controller::event)
    .update(view_controller::update)
    .run();
}
