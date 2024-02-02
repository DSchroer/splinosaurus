use crate::types::Scalar;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct StepIter<T> {
    step: T,
    position: T,
    end: T,
}

impl<T: Copy> StepIter<T> {
    pub fn new(step: T, range: RangeInclusive<T>) -> Self {
        Self {
            step,
            position: *range.start(),
            end: *range.end(),
        }
    }
}

impl<T: Scalar> Iterator for StepIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        self.position += self.step;

        if position <= self.end && self.position > self.end {
            Some(self.end)
        } else if position < self.end {
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
        let step = StepIter::new(0.5, 0.0..=1.0);
        assert_eq!(vec![0.0, 0.5, 1.0], step.collect::<Vec<_>>())
    }
}
