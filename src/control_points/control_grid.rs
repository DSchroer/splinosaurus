use crate::grid::Grid;
use crate::surfaces::UV;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct ControlGrid<T> {
    degree: usize,
    points: Grid<T>,
    u_wrapping: bool,
    v_wrapping: bool,
}

impl<T> ControlGrid<T> {
    pub fn new(degree: usize, u_len: usize, points: Vec<T>) -> Self {
        Self {
            degree,
            points: Grid::new(u_len, points),
            u_wrapping: false,
            v_wrapping: false,
        }
    }

    pub fn u_len(&self) -> usize {
        if !self.u_wrapping {
            self.points.len()
        } else {
            self.points.len() + self.degree
        }
    }

    pub fn v_len(&self) -> usize {
        if !self.v_wrapping {
            self.points.height()
        } else {
            self.points.height() + self.degree
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn set_degree(&mut self, degree: usize) {
        self.degree = degree
    }

    pub fn points(&self) -> &[T] {
        self.points.as_ref()
    }

    pub fn points_mut(&mut self) -> &mut [T] {
        self.points.as_mut()
    }

    pub fn u_wrapping(&self) -> bool {
        self.u_wrapping
    }

    pub fn set_u_wrapping(&mut self, u_wrapping: bool) {
        self.u_wrapping = u_wrapping
    }

    pub fn v_wrapping(&self) -> bool {
        self.v_wrapping
    }

    pub fn set_v_wrapping(&mut self, v_wrapping: bool) {
        self.v_wrapping = v_wrapping
    }
}

impl<T> Index<UV<usize>> for ControlGrid<T> {
    type Output = T;

    fn index(&self, (u, v): UV<usize>) -> &Self::Output {
        let u = if self.u_wrapping { u % self.u_len() } else { u };
        let v = if self.v_wrapping { v % self.v_len() } else { v };

        &self.points[(u, v)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;

    #[test]
    pub fn it_has_lengths() {
        let cg = ControlGrid::new(0, 2, vec![Vector1::new(0), Vector1::new(0)]);

        assert_eq!(2, cg.u_len());
        assert_eq!(1, cg.v_len());
    }

    #[test]
    pub fn it_indexes() {
        let cg = ControlGrid::new(
            0,
            2,
            vec![
                Vector1::new(0),
                Vector1::new(1),
                Vector1::new(2),
                Vector1::new(3),
            ],
        );

        assert_eq!(Vector1::new(0), cg[(0, 0)]);
        assert_eq!(Vector1::new(1), cg[(1, 0)]);
        assert_eq!(Vector1::new(2), cg[(0, 1)]);
        assert_eq!(Vector1::new(3), cg[(1, 1)]);
    }
}
