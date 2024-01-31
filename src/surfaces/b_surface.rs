use crate::control_points::ControlGrid;
use crate::surfaces::{Surface, UV};
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct BSurface<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: ControlGrid<D, T>,
    u_knots: Vec<usize>,
    v_knots: Vec<usize>,
}

impl<D: Dim, T: Scalar> Surface<D, T> for BSurface<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn u_range(&self) -> RangeInclusive<usize> {
        todo!()
    }

    fn v_range(&self) -> RangeInclusive<usize> {
        todo!()
    }

    fn at(&self, uv: UV<T>) -> Vector<D, T> {
        todo!()
    }
}
