use nalgebra::{Vector2, Vector3};
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::splines::{ BSpline, NURBS, Spline };

fn main() {
    let bspline = BSpline::new(
        3,
        vec![
            Vector2::new(10., 10.),
            Vector2::new(10., 300.),
            Vector2::new(300., 300.),
            Vector2::new(300., 10.),
        ],
        vec![0., 0., 0.,0., 2., 2., 2., 2.],
    );

    let nurbs = NURBS::new(
        3,
        vec![
            Vector3::new(10., 10., 1.),
            Vector3::new(10., 300., 1.),
            Vector3::new(300., 300., 1.),
            Vector3::new(300., 10., 1.),
        ],
        vec![0., 0., 0., 0., 2., 2., 2., 2.],
    );

    let canvas = Canvas::new(512, 512).title("Tile");

    // The canvas will render for you at up to 60fps.
    canvas.render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in bspline.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(255, 0, 0));
        }

        for p in nurbs.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(0, 255, 0));
        }

        for point in bspline.control_points() {
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
