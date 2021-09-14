use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let rad = 100.0;
    let n = 600;
    for i in 0..n {
        draw.z_turns((i as f32) / ((n - 1) as f32))
            .ellipse()
            .radius(3.0)
            .x_y(rad, rad)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
