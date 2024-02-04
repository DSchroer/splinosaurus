use crate::algorithms::cox_de_boor_uv;
use crate::surfaces::{BSurface, Surface, UV};
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimName, DimSub, U1};
use std::ops::RangeInclusive;

/// NURBS (surface). Representing a weighted BSurface.
#[derive(Debug)]
pub struct NURBSurface<'a, D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    spline: &'a BSurface<D, T>,
}

impl<'a, D: Dim, T: Scalar> NURBSurface<'a, D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub(crate) fn new(spline: &'a BSurface<D, T>) -> Self {
        Self { spline }
    }
}

impl<D: Dim + DimSub<U1>, T: Scalar> Surface<DimDiff<D, U1>, T> for NURBSurface<'_, D, T>
where
    <D as DimSub<Const<1>>>::Output: DimName,
    DefaultAllocator: Allocator<T, D>,
    DefaultAllocator: Allocator<T, <D as DimSub<Const<1>>>::Output>,
    <DefaultAllocator as Allocator<T, D>>::Buffer: Default,
    <DefaultAllocator as Allocator<T, <D as DimSub<Const<1>>>::Output>>::Buffer: Default,
{
    fn u_range(&self) -> RangeInclusive<T> {
        self.spline.u_range()
    }

    fn v_range(&self) -> RangeInclusive<T> {
        self.spline.v_range()
    }

    fn at(&self, uv: UV<T>) -> Vector<DimDiff<D, U1>, T> {
        let higher = cox_de_boor_uv(
            uv,
            self.spline.degree(),
            &self.spline.u_knots(),
            &self.spline.v_knots(),
            |i| {
                let mut weighted = self.spline.control_grid()[i].clone();
                for i in 0..weighted.len() - 1 {
                    let w = weighted[i] * weighted[weighted.len() - 1];
                    weighted[i] = w;
                }
                weighted
            },
        );
        let mut lower = Vector::<DimDiff<D, U1>, T>::default();
        for i in 0..lower.len() {
            lower[i] = higher[i] / higher[higher.len() - 1];
        }
        lower
    }
}
