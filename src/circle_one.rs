use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let n = 100;
    let rad = 50.0;
    let pts = (0..=n).map(|i| {
        let i = i as f32;
        let a = std::f32::consts::TAU / (n as f32) * i;
        (a.cos() * rad, a.sin() * rad)
    });

    draw.polyline().points(pts);

    draw.to_frame(app, &frame).unwrap();
}
