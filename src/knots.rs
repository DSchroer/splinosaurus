use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut, RangeInclusive};

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
    /// Create a new knot vector of degree.
    pub fn new(degree: usize, knot_vec: T) -> Self {
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

    /// Pinch a section of the knot vector.
    /// ```
    /// use splinosaurus::knots::KnotsMut;
    /// let mut vec: Vec<usize> = vec![0,0,0,1,2,3,3,3];
    ///
    /// let mut k = KnotsMut::new(2, &mut vec);
    /// k.pinch(3, 1);
    ///
    /// assert_eq!(vec![0,0,0,1,1,2,2,2], vec);
    /// ```
    pub fn pinch(&mut self, index: usize, length: usize) {
        for i in 1..=length {
            self[index + i] = self[index]
        }
        for i in index + length + 1..self.knot_vec.len() {
            self[i] = self[i] - length;
        }
    }
}

impl<'a> IndexMut<usize> for KnotsMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.knot_vec[index]
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
                let range = self.range();
                if u == *range.end() {
                    // if we have the maximum u value then handle that as a special case;
                    // look backward through the knots until we find one which is less
                    // than the maximum u value
                    self.knot_vec
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|&item| item.1 < &u)
                        .unwrap()
                        .0
                } else {
                    // perform a binary search to find the correct knot span
                    let mut low: usize = 0;
                    let mut high: usize = self.knot_vec.len() - 1;
                    let mut mid: usize = (low + high) / 2;

                    while u < self.knot_vec[mid] || u >= self.knot_vec[mid + 1] {
                        if u < self.knot_vec[mid] {
                            high = mid;
                        } else {
                            low = mid;
                        }
                        mid = (low + high) / 2;
                    }

                    mid
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    pub fn it_finds_knot_spans() {
        let vec: Vec<usize> = vec![2, 2, 2, 3, 3, 4, 4, 4];

        let k = Knots::new(2, vec.as_slice());

        assert_eq!(2, k.find_span(2));
        assert_eq!(4, k.find_span(3));
        assert_eq!(4, k.find_span(4));
    }
}
