use nalgebra::Vector3;
use obj::{Group, IndexTuple, ObjData, Object, SimplePolygon};
use splinosaurus::control_points::ControlGrid;
use splinosaurus::export::Triangulation;
use splinosaurus::surfaces::BSurface;

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
    grid.set_u_wrapping(true);

    let surface = BSurface::new(grid);

    let t = Triangulation::new(0.1, &surface);

    let mut data = ObjData::default();
    data.position = Vec::from_iter(t.points().iter().map(|p| [p.x, p.y, p.z]));
    data.objects.push(Object {
        name: "surface".to_string(),
        groups: vec![Group {
            name: "mesh".to_string(),
            index: 0,
            material: None,
            polys: Vec::from_iter(t.indexed_triangles().iter().map(|t| {
                SimplePolygon(vec![
                    IndexTuple(t[0], None, None),
                    IndexTuple(t[1], None, None),
                    IndexTuple(t[2], None, None),
                ])
            })),
        }],
    });

    data.save("a.obj").unwrap();
}
