use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    len: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(len: usize, values: Vec<T>) -> Self {
        assert_eq!(
            0,
            values.len() % len,
            "points length must be a multiple of u_len"
        );

        Self { len, values }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.len
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        assert!(
            col < self.len() || row < self.height(),
            "uv index out of bounds ({col},{row}) out of ({},{})",
            self.len(),
            self.height()
        );

        &self.values[(row * self.len) + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        assert!(
            col < self.len() || row < self.height(),
            "uv index out of bounds ({col},{row}) out of ({},{})",
            self.len(),
            self.height()
        );

        &mut self.values[(row * self.len) + col]
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
