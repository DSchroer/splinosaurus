use crate::types::Scalar;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct StepIter<T> {
    step: T,
    position: T,
    remaining: usize,
}

impl<T: Scalar> StepIter<T> {
    pub fn new(step: T, range: RangeInclusive<T>) -> Self {
        Self {
            step,
            position: *range.start(),
            remaining: ((*range.end() - *range.start()) / step).cast() + 1,
        }
    }
}

impl<T: Scalar> Iterator for StepIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        self.position += self.step;

        if self.remaining > 0 {
            self.remaining -= 1;
            Some(position)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_iters_smoothly() {
        let step = StepIter::new(0.1, 1.0..=2.0);
        assert_eq!(vec![0.0, 0.5, 1.0], step.collect::<Vec<_>>())
    }
}
