use std::ops::Index;

#[derive(Debug, Clone)]
pub struct ControlVec<T> {
    degree: usize,
    points: Vec<T>,
    wrapping: bool,
}

impl<T> ControlVec<T> {
    pub fn new(degree: usize, points: Vec<T>) -> Self {
        Self {
            degree,
            points,
            wrapping: false,
        }
    }

    pub fn new_wrapping(degree: usize, points: Vec<T>) -> Self {
        Self {
            degree,
            points,
            wrapping: true,
        }
    }

    pub fn len(&self) -> usize {
        if !self.wrapping {
            self.points.len()
        } else {
            self.points.len() + self.degree
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn set_degree(&mut self, degree: usize) {
        self.degree = degree
    }

    pub fn points(&self) -> &[T] {
        &self.points
    }

    pub fn points_mut(&mut self) -> &mut [T] {
        &mut self.points
    }

    pub fn wrapping(&self) -> bool {
        self.wrapping
    }

    pub fn set_wrapping(&mut self, wrapping: bool) {
        self.wrapping = wrapping
    }
}

impl<T> Index<usize> for ControlVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let index = if self.wrapping {
            index % self.points.len()
        } else {
            index
        };

        &self.points[index]
    }
}
