mod algorithms;
mod b_spline;
mod knot_vec;
mod nurbs;

use crate::types::{Scalar, Vector};
use nalgebra::{DefaultAllocator, Dim};
use std::marker::PhantomData;

pub use b_spline::BSpline;
pub use nurbs::NURBS;

pub trait Spline<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    fn min_u(&self) -> T;
    fn max_u(&self) -> T;
    fn at(&self, u: T) -> Vector<D, T>;

    fn quantize(&self, step: T) -> SplineQuantize<D, T, Self> {
        SplineQuantize {
            spline: self,
            step,
            position: self.min_u(),
            _dim: Default::default(),
        }
    }
}

pub struct SplineQuantize<'a, D: Dim, T: Scalar + 'static, TS: Spline<D, T>>
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    spline: &'a TS,
    step: T,
    position: T,
    _dim: PhantomData<D>,
}

impl<'a, D: Dim, T: Scalar, TS: Spline<D, T>> Iterator for SplineQuantize<'a, D, T, TS>
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    type Item = Vector<D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        self.position += self.step;

        if position < self.spline.max_u() && self.position >= self.spline.max_u() {
            Some(self.spline.at(self.spline.max_u()))
        } else if position < self.spline.max_u() {
            Some(self.spline.at(position))
        } else {
            None
        }
    }
}
