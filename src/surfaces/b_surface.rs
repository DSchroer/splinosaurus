use crate::control_points::ControlGrid;
use crate::knots::Knots;
use crate::surfaces::{Surface, UV};
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::RangeInclusive;

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
    pub fn new(control_points: ControlGrid<Vector<D, T>>) -> Self {
        Self {
            u_knots: Knots::generate(control_points.degree(), control_points.u_len()),
            v_knots: Knots::generate(control_points.degree(), control_points.v_len()),
            control_points,
        }
    }

    pub fn degree(&self) -> usize {
        self.control_points.degree()
    }

    pub fn u_knots(&self) -> Knots {
        Knots::new(self.degree(), &self.u_knots)
    }

    pub fn v_knots(&self) -> Knots {
        Knots::new(self.degree(), &self.v_knots)
    }

    pub fn control_grid(&self) -> &ControlGrid<Vector<D, T>> {
        &self.control_points
    }

    pub fn control_points(&self) -> &[Vector<D, T>] {
        self.control_points.points()
    }

    pub fn control_points_mut(&mut self) -> &mut [Vector<D, T>] {
        self.control_points.points_mut()
    }
}

impl<D: Dim, T: Scalar> Surface<D, T> for BSurface<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn u_range(&self) -> RangeInclusive<T> {
        let r = self.u_knots().range();
        T::cast_from(*r.start())..=T::cast_from(*r.end())
    }

    fn v_range(&self) -> RangeInclusive<T> {
        let r = self.v_knots().range();
        T::cast_from(*r.start())..=T::cast_from(*r.end())
    }

    fn at(&self, _uv: UV<T>) -> Vector<D, T> {
        todo!()
    }
}
