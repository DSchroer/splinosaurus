mod b_surface;
mod t_spline;

use crate::types::{Scalar, Vector};
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

use crate::step_iter::StepIter;
pub use b_surface::BSurface;

pub type UV<T> = (T, T);

pub trait Surface<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    fn u_range(&self) -> RangeInclusive<T>;
    fn v_range(&self) -> RangeInclusive<T>;
    fn at(&self, uv: UV<T>) -> Vector<D, T>;

    fn quantize_range(&self, step: T) -> impl Iterator<Item = UV<T>> {
        StepIter::new(step, self.u_range())
            .flat_map(move |u| StepIter::new(step, self.v_range()).map(move |v| (u, v)))
    }

    fn quantize(&self, step: T) -> impl Iterator<Item = Vector<D, T>> {
        self.quantize_range(step).map(|uv| self.at(uv))
    }
}
