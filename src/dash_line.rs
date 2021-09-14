use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    dash_line(&draw, Vec2::new(0., 0.), Vec2::new(300., 300.), 10.0);
    draw.to_frame(app, &frame).unwrap();
}

fn dash_line(draw: &Draw, start: Vec2, end: Vec2, step: f32) {
    let dir = end - start;
    let dir = dir.normalize();
    let mut temp_pt = start;
    let mut last_pt = temp_pt;

    while start.distance(temp_pt) < start.distance(end) {
        temp_pt += dir * step;
        if start.distance(temp_pt) > start.distance(end) {
            temp_pt = end;
        }
        draw.line()
            .start(last_pt)
            .end(temp_pt)
            .weight(1.0)
            .color(BLACK);
        temp_pt += dir * step;
        last_pt = temp_pt;
    }
}
