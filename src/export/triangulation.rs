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
        let mut normals = Vec::new();
        let mut triangles = Vec::new();

        for (y, v) in v_steps.enumerate() {
            for (x, u) in u_steps.clone().enumerate() {
                let uv = (u, v);
                points.push(surface.at(uv));

                if x > 1 && y > 1 {
                    let (a, b) = Self::tris_from_square(&mut points, y, x);

                    triangles.push(a);
                    normals.push(Self::normal_for(&points, a));

                    triangles.push(b);
                    normals.push(Self::normal_for(&points, b));
                }
            }
        }

        Self {
            points: points.into(),
            normals,
            indexed_triangles: triangles,
        }
    }

    fn tris_from_square(
        points: &mut Grid<Vector<Const<3>, T>>,
        y: usize,
        x: usize,
    ) -> (IndexedTriangle, IndexedTriangle) {
        // tri layout
        // a - b
        // | \ |
        // c - d
        let a = points.vec_index((x, y));
        let b = points.vec_index((x - 1, y));
        let c = points.vec_index((x, y - 1));
        let d = points.vec_index((x - 1, y - 1));

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