mod camera;

use crate::model;

use nannou::prelude::*;
use model::Model;
use camera::Camera;

pub fn nannou_model(app: &App) -> NannouModel {
    setup_logger().unwrap_or_else(|e| println!("Logger failed: {e}"));

    let window = app.new_window().view(view).build().unwrap();
    let view_controller = ViewController::new(window);

    NannouModel { view_controller }
}

fn view(app: &App, nannou_model: &NannouModel, frame: Frame) {
    nannou_model.view_controller.view(app, frame);
}

pub fn update(app: &App, nannou_model: &mut NannouModel, update: Update) {
    nannou_model.view_controller.update(app, update);
}

pub fn event(_app: &App, nannou_model: &mut NannouModel, event: Event) {
    nannou_model.view_controller.event(event);
}

pub struct NannouModel {
    view_controller: ViewController,
}

struct ViewController {
    _window: WindowId,
    model: Model,
    camera: Camera,
}

impl ViewController {
    pub fn new(window: WindowId) -> Self { Self { _window: window, model: Model::new(), camera: Camera::new() } }

    pub fn view(&self, app: &App, frame: Frame) {
        self.camera.view(app, frame, &self.model)
    }

    pub fn update(&mut self, app: &App, _update: Update) {
        let delta = app.duration.since_prev_update.as_secs_f32();
        self.model.update(delta);
    }

    pub fn event(&mut self, event: Event) {
        if let Event::WindowEvent { simple, .. } = event {
            if let Some(simple) = simple {
                match simple {
                    KeyPressed(key) => {
                        let camera = &mut self.camera;
                        let mut position = camera.position();
                        match key {
                            Key::Right => {
                                position.x += 20. / camera.zoom();
                            }
                            Key::Left => {
                                position.x -= 20. / camera.zoom();
                            }
                            Key::Up => {
                                position.y += 20. / camera.zoom();
                            }
                            Key::Down => {
                                position.y -= 20. / camera.zoom();
                            }
                            Key::Y => {
                                camera.set_zoom(camera.zoom() * 4. / 3.);
                            }
                            Key::T => {
                                camera.set_zoom(camera.zoom() * 3. / 4.);
                            }
                            _ => ()
                        }
                        camera.set_position(position);
                    }
                    _ => ()
                }
            }
        }
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                fern::colors::ColoredLevelConfig::new().color(record.level()),
                message
            ))
        })
        .level_for("gravity", log::LevelFilter::Debug)
        .level(log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
