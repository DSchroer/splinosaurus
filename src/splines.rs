mod b_spline;
mod nurbs;

use crate::types::{Scalar, Vector};
use core::ops::RangeInclusive;
use nalgebra::{DefaultAllocator, Dim};

use crate::step_iter::StepIter;
pub use b_spline::BSpline;
pub use nurbs::NURBSpline;

/// A single dimensional spline.
pub trait Spline<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    /// Usable range of values.
    fn range(&self) -> RangeInclusive<T>;
    /// Point at position `u`.
    fn at(&self, u: T) -> Vector<D, T>;

    /// All the steps along the spline.
    fn quantize_range(&self, step: T) -> impl ExactSizeIterator<Item = T> + Clone {
        StepIter::new(step, self.range())
    }

    /// All the points along the spline.
    fn quantize(&self, step: T) -> impl ExactSizeIterator<Item = Vector<D, T>> + Clone {
        self.quantize_range(step).map(|i| self.at(i))
    }
}
