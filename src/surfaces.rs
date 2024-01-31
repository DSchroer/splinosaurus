mod b_surface;

use crate::types::{Scalar, Vector};
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

pub use b_surface::BSurface;

type UV<T> = (T, T);

pub trait Surface<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    fn u_range(&self) -> RangeInclusive<usize>;
    fn v_range(&self) -> RangeInclusive<usize>;
    fn at(&self, uv: UV<T>) -> Vector<D, T>;
}
