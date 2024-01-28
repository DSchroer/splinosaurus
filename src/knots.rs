use crate::types::Scalar;
use az::Cast;
use std::fmt::Debug;

pub trait Knots: AsRef<[usize]> + Sized {
    fn min_u(&self) -> &usize {
        self.as_ref().first().unwrap()
    }
    fn max_u(&self) -> &usize {
        self.as_ref().last().unwrap()
    }

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn find_span(&self, u: usize) -> usize {
        debug_assert!(
            u >= *self.min_u(),
            "parameter u={:?} is below the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );
        debug_assert!(
            u <= *self.max_u(),
            "parameter u={:?} is above the required range {:?} <= u <= {:?}",
            u,
            self.min_u(),
            self.max_u()
        );

        let mut low: usize = 0;
        let mut high: usize = self.len() - 1;
        let mut mid: usize = (low + high) / 2;

        while (u < self.as_ref()[mid] || u >= self.as_ref()[mid + 1]) && mid < self.len() - 2 {
            if u < self.as_ref()[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }

        mid
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Clamped(Vec<usize>);

impl Clamped {
    pub fn new_with_clamps(degree: usize, mut v: Vec<usize>) -> Self {
        for _ in 0..degree - 1 {
            v.insert(0, v[0]);
        }
        for _ in 0..degree - 1 {
            v.push(v[v.len() - 1]);
        }
        Self(v)
    }
}

impl Clamped {
    pub fn new_uniform(degree: usize, num_points: usize) -> Self {
        let vec_size = degree + num_points + 1;
        let mut data = Vec::with_capacity(vec_size);
        let knots = vec_size - (degree * 2);
        for _ in 0..degree {
            data.push(0.cast())
        }
        for i in 0..knots {
            data.push(i)
        }
        for _ in 0..degree {
            data.push(knots - 1)
        }
        debug_assert_eq!(vec_size, data.len());
        Self(data)
    }
}

impl AsRef<[usize]> for Clamped {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}

impl Knots for Clamped {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Open(Vec<usize>);

impl Open {
    pub fn new_uniform(degree: usize, num_points: usize) -> Self {
        let vec_size = degree + num_points + 1;
        let mut data = Vec::with_capacity(vec_size);
        for i in 0..vec_size {
            data.push(i)
        }
        debug_assert_eq!(vec_size, data.len());
        Self(data)
    }
}

impl AsRef<[usize]> for Open {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}

impl Knots for Open {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_clamps_ends() {
        let c = Clamped::new_with_clamps(3, vec![1, 2, 3]);
        assert_eq!(&[1, 1, 1, 2, 3, 3, 3], c.as_ref());
    }

    #[test]
    pub fn it_can_create_uniform_clamped() {
        let c = Clamped::new_uniform(2, 4);
        assert_eq!(&[0, 0, 0, 1, 2, 2, 2], c.as_ref());
    }

    #[test]
    pub fn it_can_create_uniform_open() {
        let c = Open::new_uniform(2, 3);
        assert_eq!(&[0, 1, 2, 3, 4, 5], c.as_ref());
    }
}
