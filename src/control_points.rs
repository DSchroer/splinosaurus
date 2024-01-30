use crate::types::Vector;
use nalgebra::{DefaultAllocator, Dim, Scalar};
use std::ops::Index;

pub trait ControlPoints<T>: Index<usize, Output = T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<D: Dim, T: Scalar> ControlPoints<Vector<D, T>> for Vec<Vector<D, T>>
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
