use crate::splines::algorithms::cox_de_boor;
use crate::splines::knot_vec::KnotVec;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

pub struct BSpline<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: Vec<Vector<D, T>>,
    knots: KnotVec<T>,
    degree: usize,
}

impl<D: Dim, T: Scalar> BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(degree: usize, control_points: Vec<Vector<D, T>>, knots: Vec<T>) -> Self {
        assert_ne!(0, degree, "degree must be positive");
        assert!(
            degree < control_points.len(),
            "insufficient control points, must have at least {}",
            degree + 1
        );

        let expected_knots = degree + control_points.len() + 1;
        assert_eq!(
            knots.len(),
            expected_knots,
            "insufficient knots, must have {expected_knots}"
        );

        let knots: KnotVec<T> = knots.into();
        assert!(knots.is_clamped(degree));

        Self {
            degree,
            control_points,
            knots,
        }
    }

    pub fn knots(&self) -> &KnotVec<T> {
        &self.knots
    }

    pub fn control_points(&self) -> &[Vector<D, T>] {
        &self.control_points
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl<D: Dim, T: Scalar> Spline<D, T> for BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn min_u(&self) -> T {
        self.knots.min_u()
    }

    fn max_u(&self) -> T {
        self.knots.max_u()
    }

    fn at(&self, u: T) -> Vector<D, T> {
        cox_de_boor(u, self.degree, &self.knots, &self.control_points)
    }
}
