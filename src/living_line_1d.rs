use nannou::prelude::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    pts: VecDeque<f32>,
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

    let pts = Vec::from(model.pts.to_owned());
    let pts = pts
        .chunks_exact(2)
        .filter_map(|pt| {
            if let [x, y] = pt {
                Some((*x, *y))
            } else {
                None
            }
        })
        .collect::<Vec<(f32, f32)>>();
    draw.polyline().weight(1.0).points(pts);

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    while model.pts.len() > 200 {
        model.pts.pop_back();
    }
    if model.pts.len() == 200 {
        model.pts.pop_back();
        model.pts.pop_back();
    }
    model.pts.push_front(pos.y);
    model.pts.push_front(pos.x);
}
