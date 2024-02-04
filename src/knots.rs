use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Index, RangeInclusive};

/// A knot vector.
#[derive(Debug, Clone)]
pub struct Knots<'a, T = &'a [usize]> {
    degree: usize,
    knot_vec: T,
    _phantom: PhantomData<&'a ()>,
}

/// A mutable knot vector.
pub type KnotsMut<'a> = Knots<'a, &'a mut [usize]>;

impl<'a, T> Knots<'a, T> {
    pub(crate) fn new(degree: usize, knot_vec: T) -> Self {
        Self {
            degree,
            knot_vec,
            _phantom: Default::default(),
        }
    }
}

impl Knots<'_> {
    pub(crate) fn generate(degree: usize, num_points: usize) -> Vec<usize> {
        Vec::from_iter(0..degree + num_points + 1)
    }
}

impl KnotsMut<'_> {
    /// Clamp the ends of the knot vector so that the curve touches both the start and end control points.
    pub fn clamp_ends(&mut self) {
        let range = self.range();
        for i in 0..self.degree {
            self.knot_vec[i] = *range.start();
        }
        for i in 0..self.degree {
            self.knot_vec[self.knot_vec.len() - i - 1] = *range.end();
        }
    }
}

macro_rules! impl_knots {
    ($t:ty) => {
        impl<'a> Knots<'a, $t> {
            /// The range of a knot vector
            pub fn range(&self) -> RangeInclusive<usize> {
                self.knot_vec[self.degree]..=self.knot_vec[self.knot_vec.len() - self.degree - 1]
            }

            /// Find a span within the knot vector
            pub fn find_span(&self, u: usize) -> usize {
                let span = self.knot_vec.partition_point(|x| x < &u);
                let range = self.range();
                span.clamp(*range.start(), range.end() - 1)
            }
        }

        impl<'a> Index<usize> for Knots<'a, $t> {
            type Output = usize;

            fn index(&self, index: usize) -> &Self::Output {
                &self.knot_vec[index]
            }
        }
    };
}

impl_knots!(&'a [usize]);
impl_knots!(&'a mut [usize]);
