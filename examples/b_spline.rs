use nalgebra::Vector3;
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::control_points::ControlPoints;
use splinosaurus::splines::{BSpline, Spline, NURBS};

fn main() {
    let points = ControlPoints::new(
        2,
        vec![
            Vector3::new(100., 100., 1.),
            Vector3::new(100., 400., 5.),
            Vector3::new(400., 400., 1.),
            Vector3::new(400., 100., 3.),
        ],
    );
    let spline = BSpline::new(points);

    println!("rendering {:?}", spline);
    let canvas = Canvas::new(512, 512).title("Tile");

    // The canvas will render for you at up to 60fps.
    canvas.render_on_change(true).render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in spline.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(255, 0, 0));
        }

        let nurbs = NURBS::new(&spline);
        for p in nurbs.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(0, 255, 0));
        }

        for point in spline.control_points() {
            drawer.point(point.x as usize, point.y as usize, 5, Color::BLACK);
        }
    });
}

struct Drawer<'a> {
    image: &'a mut Image,
}

impl<'a> Drawer<'a> {
    pub fn point(&mut self, x: usize, y: usize, size: usize, color: Color) {
        for i in 0..size {
            for j in 0..size {
                self.image[XY(x + i, y + j)] = color
            }
        }
    }
}
