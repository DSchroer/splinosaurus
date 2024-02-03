use nalgebra::{Vector2, Vector3};
use pixel_canvas::{Canvas, Color, Image, XY};
use splinosaurus::control_points::{ControlGrid, ControlVec};
use splinosaurus::export::Triangulation;
use splinosaurus::splines::{BSpline, Spline};
use splinosaurus::surfaces::{BSurface, Surface};
use std::fs::File;

fn main() {
    let mut grid = ControlGrid::new(
        2,
        3,
        vec![
            //
            Vector3::new(100., 100., 100.),
            Vector3::new(200., 100., 100.),
            Vector3::new(300., 100., 100.),
            //
            Vector3::new(100., 200., 100.),
            Vector3::new(200., 200., 300.),
            Vector3::new(300., 200., 100.),
            //
            Vector3::new(100., 300., 100.),
            Vector3::new(200., 300., 100.),
            Vector3::new(300., 500., 100.),
        ],
    );

    let surface = BSurface::new(grid);

    let t = Triangulation::new(0.1, &surface);

    let mut out = File::create("a.stl").unwrap();
    stl_io::write_stl(
        &mut out,
        t.triangles_with_normals().map(|(t, n)| stl_io::Triangle {
            normal: stl_io::Vector::new([n.x, n.y, n.z]),
            vertices: t.map(|v| stl_io::Vertex::new([v.x, v.y, v.z])),
        }),
    )
    .unwrap();

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
