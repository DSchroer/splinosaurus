use crate::algorithms::cox_de_boor;
use crate::knots::Knots;
use crate::splines::Spline;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct BSpline<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: Vec<Vector<D, T>>,
    knots: Vec<usize>,
    degree: usize,
}

impl<D: Dim, T: Scalar> BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(degree: usize, control_points: Vec<Vector<D, T>>) -> Self {
        assert_ne!(0, degree, "degree must be positive");
        assert!(
            degree < control_points.len(),
            "insufficient control points, must have at least {}",
            degree + 1
        );

        Self {
            degree,
            knots: Knots::generate(degree, control_points.len()),
            control_points,
        }
    }

    pub fn knots(&self) -> Knots {
        Knots::new(&self.degree, &self.knots)
    }

    pub fn knots_mut(&mut self) -> Knots<&mut [usize]> {
        Knots::new(&self.degree, &mut self.knots)
    }

    pub fn control_points(&self) -> &[Vector<D, T>] {
        &self.control_points
    }

    pub fn control_points_mut(&mut self) -> &mut [Vector<D, T>] {
        &mut self.control_points
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl<D: Dim, T: Scalar> Spline<D, T> for BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn range(&self) -> RangeInclusive<usize> {
        self.knots().range()
    }

    fn at(&self, u: T) -> Vector<D, T> {
        cox_de_boor(u, self.degree, self.knots(), &self.control_points)
    }
}
