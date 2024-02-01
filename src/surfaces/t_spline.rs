use crate::grid::Grid;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

#[derive(Debug, Clone)]
struct Edge;

#[derive(Debug, Clone)]
struct Vertex<T> {
    position: T,
}

#[derive(Debug, Clone)]
enum BlendDir {
    Row,
    Col,
}

#[derive(Debug, Clone)]
struct TSpline<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: Grid<Vertex<Vector<D, T>>>,
    u_grid: Grid<Edge>,
    v_grid: Grid<Edge>,
    u_knots: Vec<usize>,
    v_knots: Vec<usize>,
    knot_cols: Grid<usize>,
    knot_rows: Grid<usize>,
    blend_dir: Grid<BlendDir>,
}
