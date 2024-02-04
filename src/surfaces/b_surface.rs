use crate::algorithms::cox_de_boor_uv;
use crate::control_points::ControlGrid;
use crate::knots::{Knots, KnotsMut};
use crate::surfaces::{NURBSurface, Surface, UV};
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

/// Basis spline of two degrees.
/// https://en.wikipedia.org/wiki/B-spline
#[derive(Debug, Clone)]
pub struct BSurface<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    control_points: ControlGrid<Vector<D, T>>,
    u_knots: Vec<usize>,
    v_knots: Vec<usize>,
}

impl<D: Dim, T: Scalar> BSurface<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    /// Create a new BSurface from a grid of control points.
    pub fn new(control_points: ControlGrid<Vector<D, T>>) -> Self {
        Self {
            u_knots: Knots::generate(control_points.degree(), control_points.u_len()),
            v_knots: Knots::generate(control_points.degree(), control_points.v_len()),
            control_points,
        }
    }

    /// Degree of the surface.
    pub fn degree(&self) -> usize {
        self.control_points.degree()
    }

    /// u knots.
    pub fn u_knots(&self) -> Knots {
        Knots::new(self.degree(), &self.u_knots)
    }
    /// Mutable u knots.
    pub fn u_knots_mut(&mut self) -> KnotsMut {
        KnotsMut::new(self.degree(), &mut self.u_knots)
    }

    /// v knots.
    pub fn v_knots(&self) -> Knots {
        Knots::new(self.degree(), &self.v_knots)
    }
    /// Mutable v knots.
    pub fn v_knots_mut(&mut self) -> KnotsMut {
        KnotsMut::new(self.degree(), &mut self.v_knots)
    }

    /// The control grid.
    pub fn control_grid(&self) -> &ControlGrid<Vector<D, T>> {
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

    /// Convert an N degree BSpline into a N-1 degree NURBS.
    /// The final degree becomes the weight value.
    pub fn nurbs(&self) -> NURBSurface<D, T> {
        NURBSurface::new(self)
    }
}

impl<D: Dim, T: Scalar> Surface<D, T> for BSurface<D, T>
where
    DefaultAllocator: Allocator<T, D>,
    <DefaultAllocator as Allocator<T, D>>::Buffer: Default,
{
    fn u_range(&self) -> RangeInclusive<T> {
        let r = self.u_knots().range();
        T::cast_from(*r.start())..=T::cast_from(*r.end())
    }

    fn v_range(&self) -> RangeInclusive<T> {
        let r = self.v_knots().range();
        T::cast_from(*r.start())..=T::cast_from(*r.end())
    }

    fn at(&self, uv: UV<T>) -> Vector<D, T> {
        cox_de_boor_uv(uv, self.degree(), &self.u_knots(), &self.v_knots(), |p| {
            self.control_points[p].clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;

    #[test]
    fn it_finds_all_corners() {
        let surface = BSurface::new(ControlGrid::new(
            1,
            2,
            vec![
                Vector1::new(0.),
                Vector1::new(1.),
                Vector1::new(2.),
                Vector1::new(3.),
            ],
        ));

        assert_eq!(0., surface.at((1., 1.)).x);
        assert_eq!(1., surface.at((2., 1.)).x);
        assert_eq!(2., surface.at((1., 2.)).x);
        assert_eq!(3., surface.at((2., 2.)).x);
    }
}
