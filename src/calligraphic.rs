use nannou::prelude::*;
use std::collections::VecDeque;

use svg::node::element::{path::Data, Path};
use svg::Document;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    pts: VecDeque<Vec2>,
    offsetpts: VecDeque<Vec2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .mouse_moved(mouse_moved)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        pts: VecDeque::new(),
        offsetpts: VecDeque::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polyline().weight(1.0).points(model.pts.to_owned());

    draw.polyline()
        .weight(1.0)
        .points(model.offsetpts.to_owned());

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    while model.pts.len() >= 100 {
        model.pts.pop_back();
    }
    model.pts.push_front(pos);
    model.offsetpts = (1..model.pts.len())
        .step_by(2)
        .filter_map(|i| {
            if i + 1 < model.pts.len() - 2 {
                let dir = model.pts[i + 1] - model.pts[i];
                let dist = 1.0 / dir.length().max(0.01);
                let normal = Vec2::new(-dir.y, dir.x).normalize();
                Some(model.pts[i] + normal * dist * 20.0)
            } else {
                None
            }
        })
        .collect::<VecDeque<_>>();

    if model.offsetpts.len() > 0 {
        model.offsetpts.push_front(model.pts[0]);
        model.offsetpts.push_back(*model.pts.iter().last().unwrap());
    }
}

fn key_pressed(app: &App, model: &mut Model, _key: Key) {
    let b = app.window_rect();
    let (w, h) = b.w_h();
    let aspect = h / w;

    let mut data = Data::new();
    for (i, pt) in model.pts.iter().enumerate() {
        let x = map_range(pt.x, b.left(), b.right(), 0.0, 1000.0);
        let y = map_range(pt.y, b.left(), b.right(), 0.0, 1000.0 * aspect);
        if i > 0 {
            data = data.line_to((x, y));
        }
        data = data.move_to((x, y));
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data.close());

    let mut data = Data::new();
    for (i, pt) in model.offsetpts.iter().enumerate() {
        let x = map_range(pt.x, b.left(), b.right(), 0.0, 1000.0);
        let y = map_range(pt.y, b.left(), b.right(), 0.0, 1000.0 * aspect);
        if i > 0 {
            data = data.line_to((x, y));
        }
        data = data.move_to((x, y));
    }

    let path2 = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data.close());

    let document = Document::new()
        .set("viewBox", (0, 0, 1000, (1000.0 * aspect) as i32))
        .add(path)
        .add(path2);
    svg::save("calligraphic.svg", &document).unwrap();
}
