mod b_surface;
mod nurbs;

use crate::types::{Scalar, Vector};
use core::ops::RangeInclusive;
use nalgebra::{DefaultAllocator, Dim};

use crate::step_iter::StepIter;
pub use b_surface::BSurface;
pub use nurbs::NURBSurface;

/// 2D surface coordinate.
pub type UV<T> = (T, T);

/// 2D spline surface.
pub trait Surface<D: Dim, T: Scalar + 'static>: Sized
where
    DefaultAllocator: nalgebra::allocator::Allocator<T, D>,
{
    /// Usable range of u values.
    fn u_range(&self) -> RangeInclusive<T>;
    /// Usabel range of v values.
    fn v_range(&self) -> RangeInclusive<T>;

    /// Point at coordinate `uv`.
    fn at(&self, uv: UV<T>) -> Vector<D, T>;

    /// All the u steps along the surface.
    fn quantize_u_range(&self, step: T) -> impl ExactSizeIterator<Item = T> + Clone {
        StepIter::new(step, self.u_range())
    }

    /// All the v steps along the surface.
    fn quantize_v_range(&self, step: T) -> impl ExactSizeIterator<Item = T> + Clone {
        StepIter::new(step, self.v_range())
    }

    /// All the uv steps along the surface.
    fn quantize_range(&self, step: T) -> impl ExactSizeIterator<Item = UV<T>> + Clone {
        UVRange {
            u: StepIter::new(step, self.u_range()),
            v: StepIter::new(step, self.v_range()),
            current_u: None,
            current_v: None,
        }
    }

    /// All the points along the surface.
    fn quantize(&self, step: T) -> impl ExactSizeIterator<Item = Vector<D, T>> + Clone {
        self.quantize_range(step).map(|uv| self.at(uv))
    }
}

#[derive(Debug, Clone)]
struct UVRange<T> {
    u: StepIter<T>,
    v: StepIter<T>,
    current_u: Option<T>,
    current_v: Option<StepIter<T>>,
}

impl<T: Scalar> Iterator for UVRange<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_u.is_none() {
            self.current_u = Some(self.u.next()?);
        }

        if self.current_v.is_none() {
            self.current_v = Some(self.v.clone());
        }

        let mut next_v = self.current_v.as_mut().unwrap().next();
        if next_v.is_none() {
            self.current_u = self.u.next();
            self.current_v = Some(self.v.clone());
            next_v = self.current_v.as_mut().unwrap().next();
        }

        Some((self.current_u?, next_v?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.current_v {
            None => self.u.len() * self.v.len(),
            Some(remaining) => (self.u.len() * (self.v.len())) + remaining.len(),
        };
        (len, Some(len))
    }
}

impl<T: Scalar> ExactSizeIterator for UVRange<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_uv_len() {
        let mut iter = UVRange {
            u: StepIter::new(0.5, 1.0..=2.0),
            v: StepIter::new(0.5, 1.0..=3.0),
            current_u: None,
            current_v: None,
        };

        for i in (0..=15).rev() {
            assert_eq!(i, iter.len());
            iter.next();
        }
        assert_eq!(None, iter.next())
    }
}
