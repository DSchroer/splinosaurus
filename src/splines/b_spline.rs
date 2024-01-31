use crate::algorithms::cox_de_boor;
use crate::control_points::ControlPoints;
use crate::knots::{Knots, KnotsMut};
use crate::splines::{NURBS, Spline};
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct BSpline<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: ControlPoints<D, T>,
    knots: Vec<usize>,
}

impl<D: Dim, T: Scalar> BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(control_points: ControlPoints<D, T>) -> Self {
        Self {
            knots: Knots::generate(control_points.degree(), control_points.len()),
            control_points,
        }
    }

    pub fn knots(&self) -> Knots {
        Knots::new(self.control_points.degree(), &self.knots)
    }

    pub fn knots_mut(&mut self) -> KnotsMut {
        Knots::new(self.control_points.degree(), &mut self.knots)
    }

    pub fn control_points(&self) -> &[Vector<D, T>] {
        self.control_points.points()
    }

    pub fn control_points_mut(&mut self) -> &mut [Vector<D, T>] {
        self.control_points.points_mut()
    }

    pub fn degree(&self) -> usize {
        self.control_points.degree()
    }

    pub fn nurbs(&self) -> NURBS<D, T> {
        NURBS::new(self)
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
        cox_de_boor(u, self.degree(), self.knots(), |i| self.control_points[i].clone())
    }
}
