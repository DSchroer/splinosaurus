use crate::grid::Grid;
use crate::surfaces::Surface;
use crate::types::{Scalar, Vector};
use alloc::vec::Vec;
use nalgebra::{Const, Vector3};

type IndexedTriangle = [usize; 3];
type Triangle<T> = [Vector3<T>; 3];

/// 3D Mesh of a given surface.
#[derive(Debug, Clone)]
pub struct Triangulation<T> {
    points: Vec<Vector3<T>>,
    normals: Vec<Vector3<T>>,
    indexed_triangles: Vec<IndexedTriangle>,
}

impl<T: Scalar> Triangulation<T> {
    /// Create a new Triangulation.
    /// `step` is the amount of detail generated.
    pub fn new(step: T, surface: &impl Surface<Const<3>, T>) -> Self {
        let u_steps = surface.quantize_u_range(step);
        let v_steps = surface.quantize_v_range(step);

        let mut points = Grid::with_capacity(u_steps.len(), v_steps.len());
        let mut normals = Vec::new();
        let mut indexed_triangles = Vec::new();

        for (y, v) in v_steps.enumerate() {
            for (x, u) in u_steps.clone().enumerate() {
                let uv = (u, v);
                points.push(surface.at(uv));

                if x > 0 && y > 0 {
                    let (a, b) = Self::tris_from_square(&points, (x, y));

                    indexed_triangles.push(a);
                    normals.push(Self::normal_for(&points, a));

                    indexed_triangles.push(b);
                    normals.push(Self::normal_for(&points, b));
                }
            }
        }

        Self {
            points: points.into(),
            normals,
            indexed_triangles,
        }
    }

    fn tris_from_square(
        points: &Grid<Vector<Const<3>, T>>,
        (x, y): (usize, usize),
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

    /// Points in the triangulation.
    pub fn points(&self) -> &[Vector3<T>] {
        &self.points
    }

    /// Normals for each triangle.
    pub fn normals(&self) -> &[Vector3<T>] {
        &self.normals
    }

    /// Triangles made up of index references of points.
    pub fn indexed_triangles(&self) -> &[IndexedTriangle] {
        &self.indexed_triangles
    }

    /// Triangles made up of points.
    pub fn triangles(&self) -> impl ExactSizeIterator<Item = Triangle<T>> + '_ {
        self.indexed_triangles
            .iter()
            .map(|t| [self.points[t[0]], self.points[t[1]], self.points[t[2]]])
    }

    /// Triangles made up of points with normals.
    pub fn triangles_with_normals(
        &self,
    ) -> impl ExactSizeIterator<Item = (Triangle<T>, Vector3<T>)> + '_ {
        self.triangles()
            .enumerate()
            .map(|(i, t)| (t, self.normals[i]))
    }
}
