use crate::types::Scalar;
use std::ops::{Index, IndexMut};

#[repr(transparent)]
pub struct KnotVec<T>(Vec<T>);

impl<T: Scalar> From<Vec<T>> for KnotVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T: Scalar> KnotVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        assert!(Self::is_sorted(&vec));
        Self(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn min_u(&self) -> T {
        *self.0.first().unwrap()
    }

    pub fn max_u(&self) -> T {
        *self.0.last().unwrap()
    }

    pub fn is_clamped(&self, degree: usize) -> bool {
        if self.len() < 2 * (degree + 1) {
            false
        } else {
            // check the value of the start knots
            let start_knot = self[0];
            for i_knot in &self.0[1..degree] {
                if *i_knot != start_knot {
                    return false;
                }
            }

            // check the value of the end knots
            let end_knot = self.0.last().unwrap();
            for e_knot in &self.0[self.len() - degree - 1..self.len() - 1] {
                if e_knot != end_knot {
                    return false;
                }
            }

            // everything passed
            true
        }
    }

    pub fn find_span(&self, u: T) -> usize {
        debug_assert!(
            u >= self.min_u(),
            "parameter u={:?} is below the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );
        debug_assert!(
            u <= self.max_u(),
            "parameter u={:?} is above the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );

        if u == self.max_u() {
            // if we have the maximum u value then handle that as a special case;
            // look backward through the knots until we find one which is less
            // than the maximum u value
            self.0
                .iter()
                .enumerate()
                .rev()
                .find(|&item| item.1 < &u)
                .unwrap()
                .0
        } else {
            // perform a binary search to find the correct knot span
            let mut low: usize = 0;
            let mut high: usize = self.0.len() - 1;
            let mut mid: usize = (low + high) / 2;

            while u < self.0[mid] || u >= self.0[mid + 1] {
                if u < self.0[mid] {
                    high = mid;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }

            mid
        }
    }

    fn is_sorted(data: &[T]) -> bool {
        data.windows(2).all(|w| w[0] <= w[1])
    }
}

impl<T> Index<usize> for KnotVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> IndexMut<usize> for KnotVec<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}
