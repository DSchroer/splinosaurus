use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct ControlPoints<D: Dim, T: Scalar>
where
    DefaultAllocator: Allocator<T, D>,
{
    degree: usize,
    points: Vec<Vector<D, T>>,
    wrapping: bool,
}

impl<D: Dim, T: Scalar> ControlPoints<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    pub fn new(degree: usize, points: Vec<Vector<D, T>>) -> Self {
        Self {
            degree,
            points,
            wrapping: false,
        }
    }

    pub fn new_wrapping(degree: usize, points: Vec<Vector<D, T>>) -> Self {
        Self {
            degree,
            points,
            wrapping: true,
        }
    }

    pub fn len(&self) -> usize {
        if !self.wrapping {
            self.points.len()
        } else {
            self.points.len() + self.degree
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn set_degree(&mut self, degree: usize) {
        self.degree = degree
    }

    pub fn points(&self) -> &[Vector<D, T>] {
        &self.points
    }

    pub fn points_mut(&mut self) -> &mut [Vector<D, T>] {
        &mut self.points
    }

    pub fn wrapping(&self) -> bool {
        self.wrapping
    }

    pub fn set_wrapping(&mut self, wrapping: bool) {
        self.wrapping = wrapping
    }
}

impl<D: Dim, T: Scalar> Index<usize> for ControlPoints<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    type Output = Vector<D, T>;

    fn index(&self, index: usize) -> &Self::Output {
        if !self.wrapping {
            assert!(index < self.points.len(), "index out of bounds");
        }

        &self.points[index % self.points.len()]
    }
}
