use crate::algorithms::cox_de_boor;
use crate::knots::Knots;
use crate::splines::BSpline;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimName, DimSub, U1};

#[derive(Debug)]
pub struct NURBS<D: Dim, T: Scalar, K: Knots>
where
    DefaultAllocator: Allocator<T, D>,
{
    b_spline: BSpline<D, T, K>,
}

impl<D: Dim, T: Scalar, K: Knots> From<BSpline<D, T, K>> for NURBS<D, T, K>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn from(b_spline: BSpline<D, T, K>) -> Self {
        Self { b_spline }
    }
}

impl<D: Dim, T: Scalar, K: Knots> NURBS<D, T, K>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn control_points(&self) -> &[Vector<D, T>] {
        self.b_spline.control_points()
    }
}

impl<D: Dim + DimSub<U1>, T: Scalar, K: Knots> Spline<DimDiff<D, U1>, T> for NURBS<D, T, K>
where
    <D as DimSub<Const<1>>>::Output: DimName,
    DefaultAllocator: Allocator<T, D>,
    DefaultAllocator: Allocator<T, <D as DimSub<Const<1>>>::Output>,
    <DefaultAllocator as Allocator<T, <D as DimSub<Const<1>>>::Output>>::Buffer: Default,
{
    fn min_u(&self) -> usize {
        self.b_spline.min_u()
    }

    fn max_u(&self) -> usize {
        self.b_spline.max_u()
    }

    fn at(&self, u: T) -> Vector<DimDiff<D, U1>, T> {
        let points: Vec<Vector<D, T>> = self
            .b_spline
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
        let higher = cox_de_boor(u, self.b_spline.degree(), self.b_spline.knots(), &points);
        let mut lower = Vector::<DimDiff<D, U1>, T>::default();
        for i in 0..lower.len() {
            lower[i] = higher[i] / higher[higher.len() - 1];
        }
        lower
    }
}
