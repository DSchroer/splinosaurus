use crate::grid::Grid;
use crate::surfaces::Surface;
use crate::types::Scalar;
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
    pub fn new(step: T, surface: impl Surface<Const<3>, T>) -> Self {
        // let grid = Grid::with_capacity();
        //
        // let points = Vec::from_iter(surface.quantize(step));
        // let points_grid = Grid::new(5, points);

        todo!()
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

    pub fn triangles(&self) -> impl Iterator<Item = Triangle<T>> + '_ {
        self.indexed_triangles
            .iter()
            .map(|t| [self.points[t[0]], self.points[t[1]], self.points[t[2]]])
    }

    pub fn triangles_with_normals(&self) -> impl Iterator<Item = (Triangle<T>, Vector3<T>)> + '_ {
        self.triangles()
            .enumerate()
            .map(|(i, t)| (t, self.normals[i]))
    }
}
