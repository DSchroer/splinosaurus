use crate::algorithms::cox_de_boor;
use crate::knots::{Clamped, Knots, Open};
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use az::Cast;
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

#[derive(Debug, Clone)]
pub struct BSpline<D: Dim, T: Scalar, K: Knots>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: Vec<Vector<D, T>>,
    knots: K,
    degree: usize,
}

impl<D: Dim, T: Scalar> BSpline<D, T, Clamped>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new_uniform_clamped(degree: usize, control_points: Vec<Vector<D, T>>) -> Self {
        let knots = Clamped::new_uniform(degree, control_points.len());
        Self::new(degree, control_points, knots)
    }
}

impl<D: Dim, T: Scalar> BSpline<D, T, Open>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new_uniform_open(degree: usize, control_points: Vec<Vector<D, T>>) -> Self {
        let knots = Open::new_uniform(degree, control_points.len());
        Self::new(degree, control_points, knots)
    }
}

impl<D: Dim, T: Scalar, K: Knots> BSpline<D, T, K>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(degree: usize, control_points: Vec<Vector<D, T>>, knots: K) -> Self {
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

        Self {
            degree,
            control_points,
            knots,
        }
    }

    pub fn knots(&self) -> &K {
        &self.knots
    }

    pub fn control_points(&self) -> &[Vector<D, T>] {
        &self.control_points
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl<D: Dim, T: Scalar, K: Knots> Spline<D, T> for BSpline<D, T, K>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn min_u(&self) -> &usize {
        self.knots.min_u()
    }

    fn max_u(&self) -> &usize {
        self.knots.max_u()
    }

    fn at(&self, u: T) -> Vector<D, T> {
        cox_de_boor(u, self.degree, &self.knots, &self.control_points)
    }
}
