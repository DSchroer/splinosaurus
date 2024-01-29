use crate::types::Scalar;
use az::Cast;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

pub trait Knots {
    fn min_u(&self) -> usize;
    fn max_u(&self) -> usize;
    fn find_span(&self, u: usize) -> usize;
    fn knot(&self, index: usize) -> usize;
}

#[derive(Debug, Clone)]
pub struct KnotVec(pub Vec<usize>);

impl Knots for KnotVec {
    fn min_u(&self) -> usize {
        *self.0.first().unwrap()
    }

    fn max_u(&self) -> usize {
        *self.0.last().unwrap()
    }

    fn find_span(&self, u: usize) -> usize {
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

    fn knot(&self, index: usize) -> usize {
        self.0[index]
    }
}

#[derive(Clone)]
pub struct UniformClamped {
    degree: usize,
    points: usize,
}

impl UniformClamped {
    pub fn new(degree: usize, points: usize) -> Self {
        Self { degree, points }
    }

    fn as_vec(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for u in 0..self.points + self.degree + 1 {
            v.push(self.knot(u))
        }
        debug_assert_eq!(self.points + self.degree + 1, v.len());
        v
    }
}

impl Debug for UniformClamped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_vec()).finish()
    }
}

impl Knots for UniformClamped {
    fn min_u(&self) -> usize {
        0
    }

    fn max_u(&self) -> usize {
        (self.points + self.degree) - (self.degree * 2)
    }

    fn find_span(&self, u: usize) -> usize {
        if u == self.max_u() {
            self.degree + u - 1
        } else {
            self.degree + u
        }
    }

    fn knot(&self, index: usize) -> usize {
        if index < self.degree {
            self.min_u()
        } else if index < self.points {
            (index) - (self.degree)
        } else {
            self.max_u()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_clamps_ends() {
        let c = UniformClamped::new(3, 4);
        assert_eq!(vec![0, 0, 0, 0, 1, 1, 1, 1], c.as_vec());

        let c = UniformClamped::new(3, 5);
        assert_eq!(vec![0, 0, 0, 0, 1, 2, 2, 2, 2], c.as_vec());

        let c = UniformClamped::new(2, 3);
        assert_eq!(vec![0, 0, 0, 1, 1, 1], c.as_vec());
    }

    #[test]
    pub fn it_finds_spans_in_clamp() {
        let c = UniformClamped::new(3, 4);
        assert_eq!(3, c.find_span(0));
        assert_eq!(3, c.find_span(1));
    }
}
