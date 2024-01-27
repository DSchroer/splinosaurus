use crate::splines::knot_vec::KnotVec;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

pub fn cox_de_boor<D: Dim, T: Scalar>(
    u: T,
    degree: usize,
    knots: &KnotVec<T>,
    control_points: &[Vector<D, T>],
) -> Vector<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    assert!(u >= knots.min_u() && u <= knots.max_u(), "u out of range");

    let k = knots.find_span(u);

    let mut d = Vec::with_capacity(degree + 1); // homogeneous points
    for j in 0..degree + 1 {
        let i: usize = j + k - degree;
        d.push(control_points[i].clone());
    }

    for r in 1..degree + 1 {
        for j in (r..degree + 1).rev() {
            let kp = knots[j + k - degree];
            let alpha = (u - kp) / (knots[1 + j + k - r] - kp);
            let nalpha = T::one() - alpha;
            d[j] = d[j - 1].clone() * nalpha + d[j].clone() * alpha;
        }
    }

    d[degree].clone()
}
