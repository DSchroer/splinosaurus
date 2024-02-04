use crate::algorithms::cox_de_boor_u;
use crate::splines::BSpline;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use core::ops::RangeInclusive;
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimName, DimSub, U1};

/// NURBS (spline). Representing a weighted BSpline.
#[derive(Debug)]
pub struct NURBSpline<'a, D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    spline: &'a BSpline<D, T>,
}

impl<'a, D: Dim, T: Scalar> NURBSpline<'a, D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub(crate) fn new(spline: &'a BSpline<D, T>) -> Self {
        Self { spline }
    }
}

impl<D: Dim + DimSub<U1>, T: Scalar> Spline<DimDiff<D, U1>, T> for NURBSpline<'_, D, T>
where
    <D as DimSub<Const<1>>>::Output: DimName,
    DefaultAllocator: Allocator<T, D>,
    DefaultAllocator: Allocator<T, <D as DimSub<Const<1>>>::Output>,
    <DefaultAllocator as Allocator<T, <D as DimSub<Const<1>>>::Output>>::Buffer: Default,
{
    fn range(&self) -> RangeInclusive<T> {
        self.spline.range()
    }

    fn at(&self, u: T) -> Vector<DimDiff<D, U1>, T> {
        let higher = cox_de_boor_u(u, self.spline.degree(), &self.spline.knots(), |i| {
            let mut weighted = self.spline.control_vec()[i].clone();
            for i in 0..weighted.len() - 1 {
                let w = weighted[i] * weighted[weighted.len() - 1];
                weighted[i] = w;
            }
            weighted
        });
        let mut lower = Vector::<DimDiff<D, U1>, T>::default();
        for i in 0..lower.len() {
            lower[i] = higher[i] / higher[higher.len() - 1];
        }
        lower
    }
}
