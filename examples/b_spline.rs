use nalgebra::Vector2;
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::control_points::{ControlGrid, ControlVec};
use splinosaurus::splines::{BSpline, Spline};
use splinosaurus::surfaces::{BSurface, Surface};

fn main() {
    let mut surface = BSurface::new(ControlGrid::new(
        2,
        3,
        vec![
            //
            Vector2::new(100., 100.),
            Vector2::new(200., 100.),
            Vector2::new(300., 100.),
            //
            Vector2::new(100., 200.),
            Vector2::new(200., 200.),
            Vector2::new(300., 200.),
            //
            Vector2::new(100., 300.),
            Vector2::new(200., 300.),
            Vector2::new(300., 500.),
        ],
    ));
    // surface.u_knots_mut().clamp_ends();
    // surface.v_knots_mut().clamp_ends();

    println!("rendering {:?}", surface);

    let canvas = Canvas::new(512, 512).title("Tile");

    canvas.render_on_change(true).render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in surface.quantize(0.1) {
            drawer.point(p[0] as usize, p[1] as usize, 5, Color::rgb(255, 0, 0));
        }

        for point in surface.control_points() {
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
