use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

#[derive(Debug, Clone)]
pub struct ControlGrid<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    degree: usize,
    points: Vec<Vector<D, T>>,
    u_wrapping: bool,
    v_wrapping: bool,
}
