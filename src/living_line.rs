use nannou::prelude::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    pts: VecDeque<Vec2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();
    Model {
        pts: VecDeque::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polyline().weight(1.0).points(model.pts.to_owned());

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    while model.pts.len() >= 100 {
        model.pts.pop_back();
    }
    model.pts.push_front(pos);
}
