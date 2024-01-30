use nalgebra::Vector3;
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::splines::{BSpline, Spline, NURBS};

fn main() {
    let spline: NURBS<_, _, _> = BSpline::new_uniform(
        2,
        vec![
            Vector3::new(100., 50., 1.),
            Vector3::new(200., 50. + 173.2, 1.),
            Vector3::new(300., 50., 1.),
        ],
    )
    .into();

    println!("rendering {:?}", spline);
    let canvas = Canvas::new(512, 512).title("Tile");

    // The canvas will render for you at up to 60fps.
    canvas.render_on_change(true).render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in spline.quantize(0.05) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(255, 0, 0));
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
