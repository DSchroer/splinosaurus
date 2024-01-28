use crate::types::Scalar;
use az::Cast;
use std::fmt::Debug;

pub trait Knots<T: Scalar>: Sized {
    fn as_slice(&self) -> &[T];
    fn min_u(&self) -> &T {
        self.as_slice().first().unwrap()
    }
    fn max_u(&self) -> &T {
        self.as_slice().last().unwrap()
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn find_span(&self, u: T) -> usize {
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

        if u == *self.max_u() {
            // if we have the maximum u value then handle that as a special case;
            // look backward through the knots until we find one which is less
            // than the maximum u value
            self.as_slice()
                .iter()
                .enumerate()
                .rev()
                .find(|&item| item.1 < &u)
                .unwrap()
                .0
        } else {
            // perform a binary search to find the correct knot span
            let mut low: usize = 0;
            let mut high: usize = self.len() - 1;
            let mut mid: usize = (low + high) / 2;

            while u < self.as_slice()[mid] || u >= self.as_slice()[mid + 1] {
                if u < self.as_slice()[mid] {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Clamped<T>(Vec<T>);

impl<T: Scalar> Clamped<T> {
    pub fn new_with_clamps(degree: usize, mut v: Vec<T>) -> Self {
        for _ in 0..degree - 1 {
            v.insert(0, v[0]);
        }
        for _ in 0..degree - 1 {
            v.push(v[v.len() - 1]);
        }
        Self(v)
    }
}

impl<T: Scalar> Clamped<T>
where
    usize: Cast<T>,
{
    pub fn new_uniform(degree: usize, num_points: usize) -> Self {
        let vec_size = degree + num_points + 1;
        let mut data = Vec::with_capacity(vec_size);
        let step = num_points.cast() / (vec_size - 5).cast();
        for _ in 0..degree {
            data.push(0.cast())
        }
        for i in 0..(vec_size - 4) {
            data.push(step * i.cast())
        }
        for _ in 0..degree {
            data.push(num_points.cast())
        }
        Self(data)
    }
}

impl<T: Scalar> Knots<T> for Clamped<T> {
    fn as_slice(&self) -> &[T] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_clamps_ends() {
        let c = Clamped::new_with_clamps(3, vec![1, 2, 3]);

        assert_eq!(&[1, 1, 1, 2, 3, 3, 3], c.as_slice());
    }

    #[test]
    pub fn it_can_be_created() {
        let c = Clamped::<f64>::new_uniform(2, 4);

        dbg!(&c);
        assert_eq!(&[0., 0., 0., 2., 4., 4., 4.], c.as_slice());
    }
}
