use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    len: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub(crate) fn with_capacity(len: usize, height: usize) -> Self {
        Self {
            len,
            values: Vec::with_capacity(len * height),
        }
    }

    pub fn new(len: usize, values: Vec<T>) -> Self {
        assert_eq!(
            0,
            values.len() % len,
            "points length must be a multiple of u_len"
        );

        Self { len, values }
    }

    pub(crate) fn push(&mut self, value: T) {
        self.values.push(value)
    }

    pub(crate) fn at(&self, index: usize) -> &T {
        &self.values[index]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.len
    }

    pub fn row_mut(&mut self, row: usize) -> &mut [T] {
        &mut self.values[(row * self.len)..(row * self.len) + self.len]
    }

    pub fn vec_index(&self, (col, row): (usize, usize)) -> usize {
        (row * self.len) + col
    }
}

impl<T> From<Grid<T>> for Vec<T> {
    fn from(value: Grid<T>) -> Self {
        value.values
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        assert!(
            col < self.len() && row < self.height(),
            "uv index out of bounds ({col},{row}) out of ({},{})",
            self.len(),
            self.height()
        );

        &self.values[self.vec_index((col, row))]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        assert!(
            col < self.len() && row < self.height(),
            "uv index out of bounds ({col},{row}) out of ({},{})",
            self.len(),
            self.height()
        );

        let index = self.vec_index((col, row));
        &mut self.values[index]
    }
}

impl<T> AsRef<[T]> for Grid<T> {
    fn as_ref(&self) -> &[T] {
        &self.values
    }
}

impl<T> AsMut<[T]> for Grid<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.values
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use alloc::vec;

    #[test]
    fn it_accesses_col_row() {
        let mut original = Grid::new(2, vec![1, 2, 3, 4]);

        assert_eq!(1, original[(0, 0)]);
        assert_eq!(2, original[(1, 0)]);
        assert_eq!(3, original[(0, 1)]);
        assert_eq!(4, original[(1, 1)]);

        assert_eq!(&[1, 2], original.row_mut(0));
        assert_eq!(&[3, 4], original.row_mut(1));
    }

    #[test]
    fn it_does_sub_grids() {
        let original = Grid::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut d = Grid::with_capacity(2, 2);
        for y in 0..2 {
            for x in 0..2 {
                d.push(original[(x, y)]);
            }
        }

        assert_eq!(vec![1, 2, 4, 5], d.values);
    }
}
