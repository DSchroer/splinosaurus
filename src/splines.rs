mod b_spline;
mod nurbs;

use crate::types::{Scalar, Vector};
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

use crate::step_iter::StepIter;
pub use b_spline::BSpline;
pub use nurbs::NURBS;

pub trait Spline<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    fn range(&self) -> RangeInclusive<T>;
    fn wrapping(&self) -> bool;
    fn at(&self, u: T) -> Vector<D, T>;

    fn quantize_range(&self, step: T) -> impl ExactSizeIterator<Item = T> + Clone {
        StepIter::new(step, self.range())
    }

    fn quantize(&self, step: T) -> impl ExactSizeIterator<Item = Vector<D, T>> + Clone {
        self.quantize_range(step).map(|i| self.at(i))
    }
}
