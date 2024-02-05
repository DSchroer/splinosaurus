use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

/// Axis aligned bounding box.
#[derive(Debug, Clone)]
pub struct BoundingBox<D: Dim, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    min: Vector<D, T>,
    max: Vector<D, T>,
}

impl<D: Dim, T: Scalar> BoundingBox<D, T>
where
    DefaultAllocator: Allocator<T, D>,
    <DefaultAllocator as Allocator<T, D>>::Buffer: Default,
{
    /// Calculate the bounding box for a set of points.
    pub fn new(points: &[Vector<D, T>]) -> Self {
        let mut point_iter = points.iter();

        let mut min = point_iter.next().cloned().unwrap_or_default();
        let mut max = min.clone();

        for point in point_iter {
            for i in 0..point.len() {
                if min[i] > point[i] {
                    min[i] = point[i];
                }
                if max[i] < point[i] {
                    max[i] = point[i];
                }
            }
        }

        Self { min, max }
    }

    /// Minimum corner of the bounding box.
    pub fn min(&self) -> &Vector<D, T> {
        &self.min
    }

    /// Maximum corner of the bounding box.
    pub fn max(&self) -> &Vector<D, T> {
        &self.max
    }
}
