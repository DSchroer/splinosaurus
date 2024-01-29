use nalgebra::Vector2;
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::knots::KnotVec;
use splinosaurus::splines::{BSpline, Spline};

fn main() {
    let bspline = BSpline::new_uniform_clamped(
        2,
        vec![
            Vector2::new(50., 50.),
            Vector2::new(50., 300.),
            Vector2::new(300., 300.),
            Vector2::new(300., 500.),
        ],
    );

    println!("rendering {:?}", bspline);
    let canvas = Canvas::new(512, 512).title("Tile");

    // The canvas will render for you at up to 60fps.
    canvas.render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in bspline.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(255, 0, 0));
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
