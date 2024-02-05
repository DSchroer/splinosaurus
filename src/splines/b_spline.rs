use crate::algorithms::cox_de_boor_u;
use crate::control_points::ControlVec;
use crate::knots::{Knots, KnotsMut};
use crate::splines::{NURBSpline, Spline};
use crate::types::{Scalar, Vector};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::RangeInclusive;
use nalgebra::allocator::Allocator;
use nalgebra::{Const, DefaultAllocator, Dim, Vector2, Vector3};

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

impl BSpline<Const<2>, f64> {
    /// 2D Unit square.
    pub fn square() -> Self {
        let mut control = ControlVec::new(
            1,
            vec![
                Vector2::new(-0.5, -0.5),
                Vector2::new(0.5, -0.5),
                Vector2::new(0.5, 0.5),
            ],
        );
        control.set_wrapping(true);
        Self::new(control)
    }
}

impl BSpline<Const<3>, f64> {
    /// 2D Unit circle (NURBS) of diameter 1.
    pub fn circle() -> Self {
        let arc_w = 1.0 / f64::sqrt(2.0);
        let r = 0.5;
        let mut control = ControlVec::new(
            2,
            vec![
                Vector3::new(0.0, r, 1.0),
                Vector3::new(r, r, arc_w),
                Vector3::new(r, 0.0, 1.0),
                Vector3::new(r, -r, arc_w),
                Vector3::new(0.0, -r, 1.0),
                Vector3::new(-r, -r, arc_w),
                Vector3::new(-r, 0.0, 1.0),
                Vector3::new(-r, r, arc_w),
            ],
        );
        control.set_wrapping(true);
        let mut curve = Self::new(control);
        curve.knots_mut().pinch(1, 1);
        curve.knots_mut().pinch(3, 1);
        curve.knots_mut().pinch(5, 1);
        curve.knots_mut().pinch(7, 1);
        curve.knots_mut().pinch(9, 1);
        curve
    }
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
