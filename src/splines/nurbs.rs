use crate::algorithms::cox_de_boor;
use crate::splines::BSpline;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimName, DimSub, U1};

pub struct NURBS<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    b_spline: BSpline<D, T>,
}

impl<D: Dim, T: Scalar> NURBS<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(degree: usize, control_points: Vec<Vector<D, T>>, knots: Vec<T>) -> Self {
        Self {
            b_spline: BSpline::new(degree, control_points, knots),
        }
    }
}

impl<D: Dim, T: Scalar> Spline<DimDiff<D, U1>, T> for NURBS<D, T>
where
    D: Dim + DimSub<U1>,
    <D as DimSub<Const<1>>>::Output: DimName,
    DefaultAllocator: Allocator<T, D>,
    DefaultAllocator: Allocator<T, <D as DimSub<Const<1>>>::Output>,
{
    fn min_u(&self) -> T {
        self.b_spline.min_u()
    }

    fn max_u(&self) -> T {
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
        let mut lower = Vector::<DimDiff<D, U1>, T>::zeros();
        for i in 0..lower.len() {
            lower[i] = higher[i] / higher[higher.len() - 1];
        }
        lower
    }
}
