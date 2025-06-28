use nannou::prelude::*;

pub struct Model {
    app: crate::app::App,
}

pub fn model(app: &App) -> Model {
    app
    .new_window()
    .view(view)
    .build()
    .expect("window build failed");

    Model { app: crate::app::App::new() }
}

pub fn event(_: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple, .. } = event {
        if let Some(simple) = simple {
            match simple {
                KeyPressed(key) => {
                    let camera = model.app.camera();
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

pub fn update(_: &App, model: &mut Model, update: Update) {
    model.app.update(update.since_last.as_secs_f32());
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.text("Zoom: T/Y").y(frame.rect().y.end - 10.0).font_size(20);

    for body in model.app.bodies() {
        let body = model.app.world_to_camera(body);

        draw.ellipse()
        .color(WHITE)
        .radius(body.1)
        .xy(body.0);
    }

    draw.to_frame(app, &frame).unwrap();
}
