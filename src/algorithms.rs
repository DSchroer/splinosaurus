use crate::knots::Knots;
use crate::types::{Scalar, Vector};
use az::Cast;
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

pub fn cox_de_boor<D: Dim, T: Scalar>(
    u: T,
    degree: usize,
    knots: &impl Knots,
    control_points: &[Vector<D, T>],
) -> Vector<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    assert!(
        u.cast() >= *knots.min_u() && u.cast() <= *knots.max_u(),
        "u out of range"
    );

    let k = knots.find_span(u.cast());

    let mut d = Vec::with_capacity(degree + 1); // homogeneous points
    for j in 0..degree + 1 {
        let i = (j + k).wrapping_sub(degree) % control_points.len();
        d.push(control_points[i].clone());
    }

    let knots = knots.as_ref();
    for r in 1..degree + 1 {
        for j in (r..degree + 1).rev() {
            let kp = T::cast_from(knots[(j + k).wrapping_sub(degree) % knots.len()]);
            let kp_1 = T::cast_from(knots[(1 + j + k).wrapping_sub(r) % knots.len()].cast());
            let alpha = (u - kp) / (kp_1 - kp);
            let n_alpha = T::one() - alpha;
            d[j] = d[j - 1].clone() * n_alpha + d[j].clone() * alpha;
        }
    }

    d[degree].clone()
}
