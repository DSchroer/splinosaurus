use nalgebra::Vector2;
use splinosaurus::control_points::ControlVec;
use splinosaurus::splines::{BSpline, Spline};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn main() {
    let grid = ControlVec::new(
        2,
        vec![
            Vector2::new(100., 100.),
            Vector2::new(200., 300.),
            Vector2::new(300., 100.),
            Vector2::new(400., 200.),
        ],
    );
    let spline = BSpline::new(grid);

    let mut data = Data::new();
    for (i, p) in spline.quantize(0.01).enumerate() {
        if i == 0 {
            data = data.move_to((p.x, p.y));
        } else {
            data = data.line_to((p.x, p.y));
        }
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 500, 500)).add(path);

    svg::save("image.svg", &document).unwrap();
}
