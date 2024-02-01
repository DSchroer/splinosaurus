use nalgebra::Vector2;
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::control_points::ControlGrid;
use splinosaurus::surfaces::{BSurface, Surface};

fn main() {
    let points = ControlGrid::new(
        1,
        2,
        vec![
            Vector2::new(100., 100.),
            Vector2::new(100., 400.),
            Vector2::new(400., 400.),
            Vector2::new(400., 100.),
        ],
    );
    let surface = BSurface::new(points);

    println!("rendering {:?}", surface);

    let canvas = Canvas::new(512, 512).title("Tile");

    // The canvas will render for you at up to 60fps.
    canvas.render(move |_, image| {
        image.fill(Color::WHITE);
        let mut drawer = Drawer { image };

        for p in surface.quantize(0.05) {
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
