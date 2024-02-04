use crate::algorithms::cox_de_boor_u;
use crate::control_points::ControlVec;
use crate::knots::{Knots, KnotsMut};
use crate::splines::{NURBSpline, Spline};
use crate::types::{Scalar, Vector};
use alloc::vec::Vec;
use core::ops::RangeInclusive;
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

/// Basis spline of a single degree.
/// https://en.wikipedia.org/wiki/B-spline
#[derive(Debug, Clone)]
pub struct BSpline<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: ControlVec<Vector<D, T>>,
    knots: Vec<usize>,
}

impl<D: Dim, T: Scalar> BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    /// Create a new basis spline for a list of control points.
    pub fn new(control_points: ControlVec<Vector<D, T>>) -> Self {
        Self {
            knots: Knots::generate(control_points.degree(), control_points.len()),
            control_points,
        }
    }

    /// Access the knots of the spline.
    pub fn knots(&self) -> Knots {
        Knots::new(self.control_points.degree(), &self.knots)
    }

    /// Mutable access to the knots of the spline.
    pub fn knots_mut(&mut self) -> KnotsMut {
        Knots::new(self.control_points.degree(), &mut self.knots)
    }

    /// Access to the control vec.
    pub fn control_vec(&self) -> &ControlVec<Vector<D, T>> {
        &self.control_points
    }

    /// The control points.
    pub fn control_points(&self) -> &[Vector<D, T>] {
        self.control_points.points()
    }

    /// Mutable control points.
    pub fn control_points_mut(&mut self) -> &mut [Vector<D, T>] {
        self.control_points.points_mut()
    }

    /// Degree of the curve.
    pub fn degree(&self) -> usize {
        self.control_points.degree()
    }

    /// Convert an N degree BSpline into a N-1 degree NURBS.
    /// The final degree becomes the weight value.
    pub fn nurbs(&self) -> NURBSpline<D, T> {
        NURBSpline::new(self)
    }
}

impl<D: Dim, T: Scalar> Spline<D, T> for BSpline<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn range(&self) -> RangeInclusive<T> {
        let r = self.knots().range();
        T::cast_from(*r.start())..=T::cast_from(*r.end())
    }

    fn at(&self, u: T) -> Vector<D, T> {
        cox_de_boor_u(u, self.degree(), &self.knots(), |i| {
            self.control_points[i].clone()
        })
    }
}
