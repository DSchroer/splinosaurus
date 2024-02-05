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
    pub fn new<'a>(points: impl IntoIterator<Item = &'a Vector<D, T>>) -> Self {
        let mut points = points.into_iter();

        let mut min = points.next().cloned().unwrap_or_default();
        let mut max = min.clone();

        for point in points {
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

    /// Center of the bounding box.
    pub fn center(&self) -> Vector<D, T> {
        (&self.min + &self.max) / T::cast_from(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::splines::{BSpline, Spline};
    use alloc::vec::Vec;
    use nalgebra::Vector3;

    #[test]
    fn it_calculates_for_control_points() {
        let spline = BSpline::circle();
        let bbox = BoundingBox::new(spline.control_points());

        assert_ne!(*bbox.min(), Vector3::default());
        assert_ne!(*bbox.max(), Vector3::default());
    }

    #[test]
    fn it_calculates_for_spline_points() {
        let spline = BSpline::circle();
        let points: Vec<_> = spline.quantize(0.01).collect();
        let bbox = BoundingBox::new(&points);

        assert_ne!(*bbox.min(), Vector3::default());
        assert_ne!(*bbox.max(), Vector3::default());
    }
}
