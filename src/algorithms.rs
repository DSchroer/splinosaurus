use crate::grid::Grid;
use crate::knots::Knots;
use crate::surfaces::UV;
use crate::types::{Scalar, Vector};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, Dim};

pub fn cox_de_boor_u<D: Dim, T: Scalar>(
    u: T,
    degree: usize,
    knots: &Knots,
    control_points: impl Fn(usize) -> Vector<D, T>,
) -> Vector<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    assert!(knots.range().contains(&u.cast()), "u out of range");

    let k = knots.find_span(u.cast());

    let mut d = Vec::with_capacity(degree + 1);
    for j in 0..degree + 1 {
        let i = j + k - degree;
        d.push(control_points(i));
    }

    cox_de_boor(u, degree, k, knots, &mut d);

    d.remove(degree)
}

pub fn cox_de_boor_uv<D: Dim, T: Scalar>(
    (u, v): UV<T>,
    degree: usize,
    u_knots: &Knots,
    v_knots: &Knots,
    control_points: impl Fn(UV<usize>) -> Vector<D, T>,
) -> Vector<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    assert!(u_knots.range().contains(&u.cast()), "u out of range");
    assert!(v_knots.range().contains(&v.cast()), "v out of range");

    let u_k = u_knots.find_span(u.cast());
    let v_k = v_knots.find_span(v.cast());

    let mut d = Grid::with_capacity(degree + 1, degree + 1);
    for v_j in 0..degree + 1 {
        for u_j in 0..degree + 1 {
            let u_i = u_j + u_k - degree;
            let v_i = v_j + v_k - degree;
            d.push(control_points((u_i, v_i)))
        }
    }

    for j in 0..degree + 1 {
        cox_de_boor(u, degree, u_k, u_knots, d.row_mut(j));
    }

    for r in 1..degree + 1 {
        for j in (r..degree + 1).rev() {
            let alpha = alpha(v, v_k, degree, r, j, &v_knots);
            d[(degree, j)] = &d[(degree, j - 1)] * (T::one() - alpha) + &d[(degree, j)] * alpha;
        }
    }

    d[(degree, degree)].clone()
}

fn cox_de_boor<D: Dim, T: Scalar>(
    u: T,
    degree: usize,
    k: usize,
    knots: &Knots,
    d: &mut [Vector<D, T>],
) where
    DefaultAllocator: Allocator<T, D>,
{
    for r in 1..degree + 1 {
        for j in (r..degree + 1).rev() {
            let alpha = alpha(u, k, degree, r, j, &knots);
            d[j] = &d[j - 1] * (T::one() - alpha) + &d[j] * alpha;
        }
    }
}

fn alpha<T: Scalar>(u: T, knot_span: usize, degree: usize, r: usize, j: usize, knots: &Knots) -> T {
    let kp = T::cast_from(knots[j + knot_span - degree]);
    let kp_1 = T::cast_from(knots[1 + j + knot_span - r]);
    (u - kp) / (kp_1 - kp)
}
