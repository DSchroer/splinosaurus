use std::ops::Index;

/// 1D list of control points.
#[derive(Debug, Clone)]
pub struct ControlVec<T> {
    degree: usize,
    points: Vec<T>,
    wrapping: bool,
}

impl<T> ControlVec<T> {
    /// Construct a control vec of `degree`.
    pub fn new(degree: usize, points: Vec<T>) -> Self {
        Self {
            degree,
            points,
            wrapping: false,
        }
    }

    /// Number of control points with wrapping included.
    pub fn len(&self) -> usize {
        if !self.wrapping {
            self.points.len()
        } else {
            self.points.len() + self.degree
        }
    }

    /// If there are no points.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Degree of the curve.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Set the degree of the curve.
    pub fn set_degree(&mut self, degree: usize) {
        self.degree = degree
    }

    /// Access the control points.
    pub fn points(&self) -> &[T] {
        &self.points
    }

    /// Mutable access the control points.
    pub fn points_mut(&mut self) -> &mut [T] {
        &mut self.points
    }

    /// Is wrapping.
    pub fn wrapping(&self) -> bool {
        self.wrapping
    }

    /// Set is wrapping.
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
