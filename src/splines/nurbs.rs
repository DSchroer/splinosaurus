use crate::algorithms::cox_de_boor;
use crate::splines::BSpline;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimName, DimSub, U1};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct NURBS<'a, D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub spline: &'a BSpline<D, T>,
}

impl<'a, D: Dim, T: Scalar> NURBS<'a, D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(spline: &'a BSpline<D, T>) -> Self {
        Self { spline }
    }
}

impl<D: Dim + DimSub<U1>, T: Scalar> Spline<DimDiff<D, U1>, T> for NURBS<'_, D, T>
where
    <D as DimSub<Const<1>>>::Output: DimName,
    DefaultAllocator: Allocator<T, D>,
    DefaultAllocator: Allocator<T, <D as DimSub<Const<1>>>::Output>,
    <DefaultAllocator as Allocator<T, <D as DimSub<Const<1>>>::Output>>::Buffer: Default,
{
    fn range(&self) -> RangeInclusive<usize> {
        self.spline.knots().range()
    }

    fn at(&self, u: T) -> Vector<DimDiff<D, U1>, T> {
        let points: Vec<Vector<D, T>> = self
            .spline
            .control_points()
            .iter()
            .map(|p| {
                let mut weighted = p.clone();
                for i in 0..p.len() - 1 {
                    let w = weighted[i] * weighted[weighted.len() - 1];
                    weighted[i] = w;
                }
                weighted
            })
            .collect();
        let higher = cox_de_boor(u, self.spline.degree(), self.spline.knots(), &points);
        let mut lower = Vector::<DimDiff<D, U1>, T>::default();
        for i in 0..lower.len() {
            lower[i] = higher[i] / higher[higher.len() - 1];
        }
        lower
    }
}
