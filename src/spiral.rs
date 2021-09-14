use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let n = 300;
    let rad = 5.0;
    let pts = (0..n).map(|i| {
        let i = i as f32;
        let a = 3.0 * std::f32::consts::TAU / (n as f32) * i;
        // circle involute via https://mathworld.wolfram.com/CircleInvolute.html
        ((a.cos() + a.sin() * a) * rad, (a.sin() - a.cos() * a) * rad)
    });

    draw.polyline().points(pts);

    draw.to_frame(app, &frame).unwrap();
}
