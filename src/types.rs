use nalgebra::DefaultAllocator;
use num_traits::identities::One;
use num_traits::Zero;
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
    + One
    + Zero
    + Default
    + 'static
{
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
        + One
        + Zero
        + Default
        + 'static
{
}

pub type Vector<D, T> =
    nalgebra::Vector<T, D, <DefaultAllocator as nalgebra::allocator::Allocator<T, D>>::Buffer>;
