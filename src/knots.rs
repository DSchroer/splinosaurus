use std::fmt::{Debug, Formatter};

pub trait Knots {
    fn len(&self) -> usize;
    fn min_u(&self) -> usize;
    fn max_u(&self) -> usize;
    fn find_span(&self, u: usize) -> usize;
    fn knot(&self, index: usize) -> usize;
}

fn knots_to_vec(k: &impl Knots) -> Vec<usize> {
    Vec::from_iter((0..k.len()).map(|u| k.knot(u)))
}

fn debug_knot(k: &impl Knots, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UniformClamped")
        .field("min_u", &k.min_u())
        .field("max_u", &k.max_u())
        .field("knots", &knots_to_vec(k))
        .finish()
}

#[derive(Clone)]
pub struct Uniform {
    degree: usize,
    points: usize,
}

impl Uniform {
    pub fn new(degree: usize, points: usize) -> Self {
        Self { degree, points }
    }
}

impl Debug for Uniform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_knot(self, f)
    }
}

impl Knots for Uniform {
    fn len(&self) -> usize {
        self.points + self.degree + 1
    }

    fn min_u(&self) -> usize {
        self.degree
    }

    fn max_u(&self) -> usize {
        self.points
    }

    fn find_span(&self, u: usize) -> usize {
        if u == self.max_u() {
            u - 1
        } else {
            u
        }
    }

    fn knot(&self, index: usize) -> usize {
        index
    }
}

#[derive(Clone)]
pub struct Clamped {
    degree: usize,
    points: usize,
}

impl Clamped {
    pub fn new(degree: usize, points: usize) -> Self {
        Self { degree, points }
    }
}

impl Debug for Clamped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_knot(self, f)
    }
}

impl Knots for Clamped {
    fn len(&self) -> usize {
        self.points + self.degree + 1
    }

    fn min_u(&self) -> usize {
        self.degree
    }

    fn max_u(&self) -> usize {
        self.points
    }

    fn find_span(&self, u: usize) -> usize {
        if u == self.max_u() {
            u - 1
        } else {
            u
        }
    }

    fn knot(&self, index: usize) -> usize {
        if index < self.degree {
            self.min_u()
        } else if index >= self.len() - self.degree {
            self.max_u()
        } else {
            index
        }
    }
}
