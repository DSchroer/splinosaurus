use crate::grid::Grid;
use crate::surfaces::Surface;
use crate::types::{Scalar, Vector};
use nalgebra::{Const, Vector3};

type IndexedTriangle = [usize; 3];
type Triangle<T> = [Vector3<T>; 3];

#[derive(Debug, Clone)]
pub struct Triangulation<T> {
    points: Vec<Vector3<T>>,
    normals: Vec<Vector3<T>>,
    indexed_triangles: Vec<IndexedTriangle>,
}

impl<T: Scalar> Triangulation<T> {
    pub fn new(step: T, surface: &impl Surface<Const<3>, T>) -> Self {
        let u_steps = surface.quantize_u_range(step);
        let v_steps = surface.quantize_v_range(step);

        let mut points = Grid::with_capacity(u_steps.len(), v_steps.len());
        let mut triangulation = Self {
            points: Vec::new(),
            normals: Vec::new(),
            indexed_triangles: Vec::new(),
        };

        for (y, v) in v_steps.enumerate() {
            for (x, u) in u_steps.clone().enumerate() {
                let uv = (u, v);
                points.push(surface.at(uv));

                if x > 0 && y > 0 {
                    triangulation.add_tris_and_normals(&points, (x, y), (x - 1, y - 1));
                }
            }
        }

        if surface.u_wrapping() {
            for y in 1..points.height() - 1 {
                triangulation.add_tris_and_normals(&points, (1, y + 1), (0, y));
            }
        }

        if surface.v_wrapping() {
            for x in 1..points.len() - 1 {
                triangulation.add_tris_and_normals(&points, (x + 1, 1), (x, 0));
            }
        }

        triangulation.points = points.into();
        triangulation
    }

    fn add_tris_and_normals(
        &mut self,
        points: &Grid<Vector<Const<3>, T>>,
        xy1: (usize, usize),
        xy2: (usize, usize),
    ) {
        let (a, b) = Self::tris_from_square(&points, xy1, xy2);

        self.indexed_triangles.push(a);
        self.normals.push(Self::normal_for(&points, a));

        self.indexed_triangles.push(b);
        self.normals.push(Self::normal_for(&points, b));
    }

    fn tris_from_square(
        points: &Grid<Vector<Const<3>, T>>,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> (IndexedTriangle, IndexedTriangle) {
        fn wrapping_index<T: Scalar>(
            points: &Grid<Vector<Const<3>, T>>,
            (x, y): (usize, usize),
        ) -> usize {
            let xy = (x, y);
            points.vec_index(xy)
        }

        // tri layout
        // a - b
        // | \ |
        // c - d
        let a = wrapping_index(points, (x1, y1));
        let b = wrapping_index(points, (x2, y1));
        let c = wrapping_index(points, (x1, y2));
        let d = wrapping_index(points, (x2, y2));

        ([a, d, c], [a, b, d])
    }

    fn normal_for(points: &Grid<Vector<Const<3>, T>>, triangle: IndexedTriangle) -> Vector3<T> {
        let a = points.at(triangle[1]) - points.at(triangle[0]);
        let b = points.at(triangle[2]) - points.at(triangle[0]);

        a.cross(&b)
    }

    pub fn points(&self) -> &[Vector3<T>] {
        &self.points
    }

    pub fn normals(&self) -> &[Vector3<T>] {
        &self.normals
    }

    pub fn indexed_triangles(&self) -> &[IndexedTriangle] {
        &self.indexed_triangles
    }

    pub fn triangles(&self) -> impl ExactSizeIterator<Item = Triangle<T>> + '_ {
        self.indexed_triangles
            .iter()
            .map(|t| [self.points[t[0]], self.points[t[1]], self.points[t[2]]])
    }

    pub fn triangles_with_normals(
        &self,
    ) -> impl ExactSizeIterator<Item = (Triangle<T>, Vector3<T>)> + '_ {
        self.triangles()
            .enumerate()
            .map(|(i, t)| (t, self.normals[i]))
    }
}
