use nannou::noise::NoiseFn;
use svg::node::element::{path::Data, Path};
use svg::Document;

fn main() {
    let mut document = Document::new();
    for across in 1..15 {
        let rnd = nannou::rand::random_range(1.0, 50.0);
        document = gen_thick_line(&mut document, across as f64 * 20.0, rnd as usize);
    }
    svg::save("weights.svg", &document).unwrap();
}

fn gen_thick_line(document: &mut Document, x: f64, amt: usize) -> Document {
    let mut doc = document.clone();
    let noise = nannou::noise::Worley::new();
    for ln in 1..amt {
        let mut data = Data::new().move_to((x, 10.));
        for i in 10..200 {
            let l64 = ln as f64;
            let xoff = 3.0 * noise.get([x + l64, l64 - i as f64]);
            data = data.line_to((x + xoff, i));
            data = data.move_to((x + xoff, i));
        }
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("d", data);
        doc = doc.add(path)
    }
    doc
}
