use az::Cast;
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
    + Div<Output = Self>
    + Cast<f64>
    + Default
    + 'static
{
    fn from<T: Cast<Self>>(value: T) -> Self {
        Cast::cast(value)
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
        + Div<Output = Self>
        + Cast<f64>
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

    #[test]
    fn it_casts_to_scalar() {
        fn thing(_: impl Scalar) {}

        thing(0f64);
        thing(0f32);
        thing(0f32);
        thing(fixed::FixedI64::<U3>::default());
    }
}
