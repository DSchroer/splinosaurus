use az::{Cast, CastFrom};
use nalgebra::DefaultAllocator;
use std::fmt::Debug;
use std::ops::*;

pub trait Scalar:
    Copy
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + Cast<usize>
    + CastFrom<usize>
    + Default
    + 'static
{
    fn one() -> Self {
        Self::cast_from(1)
    }
}

impl<T> Scalar for T where
    T: Copy
        + PartialOrd
        + Debug
        + Add<Output = Self>
        + AddAssign
        + Mul<Output = Self>
        + MulAssign
        + Sub<Output = Self>
        + SubAssign
        + Div<Output = Self>
        + Cast<usize>
        + CastFrom<usize>
        + Default
        + 'static
{
}

pub type Vector<D, T> =
    nalgebra::Vector<T, D, <DefaultAllocator as nalgebra::allocator::Allocator<T, D>>::Buffer>;

#[cfg(test)]
mod tests {
    use super::*;
    use fixed::types::extra::U3;
    use nalgebra::{Const, Vector2};

    #[test]
    fn it_casts_to_scalar() {
        fn thing(_: impl Scalar) {}

        thing(0f64);
        thing(0f32);
        thing(fixed::FixedI64::<U3>::default());
    }

    #[test]
    fn it_casts_to_vector() {
        fn thing(_: Vector<Const<2>, impl Scalar>) {}

        thing(Vector2::new(0f64, 0f64));
        thing(Vector2::new(0f32, 0f32));
        thing(Vector2::new(
            fixed::FixedI64::<U3>::default(),
            fixed::FixedI64::<U3>::default(),
        ));
    }
}
