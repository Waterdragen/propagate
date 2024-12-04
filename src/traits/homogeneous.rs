use crate::TwoStates;
use std::ops::ControlFlow;

/// A trait for enums that can yield an inner value when their variants are homogeneous.
///
/// The `Homogeneous` trait is designed for types where all enum variant
/// representations are the same. It provides a method to extract the inner
/// value.
///
/// # Associated Method
///
/// - `fn get_inner_value(self) -> T`: Converts the implementing type to its
///   inner value.
///
/// # Implementations
///
/// This trait is implemented for enums like `Result<T, T>`, `Option<()>`,
/// and `ControlFlow<T, T>`, allowing for seamless extraction of the inner
/// value when the variant types are homogeneous.
pub trait Homogeneous<T> {
    fn get_inner_value(self) -> T;
}

macro_rules! impl_homogeneous {
    ($(<$generic:ident>)? $ty:ident<$generic_ty:ty $(, $second:ty)?> $($borrow:tt)*) => {
        impl <'a, $($generic)?> Homogeneous<$($borrow)* $generic_ty> for $($borrow)* $ty<$generic_ty $(, $second)?> {
            #[inline]
            fn get_inner_value(self) -> $($borrow)* $generic_ty {
                TwoStates::two_states(self).unwrap_or_else(|v| v)
            }
        }
    };
}

impl_homogeneous!(<T> Result<T, T>);
impl_homogeneous!(<T> Result<T, T> &'a);
impl_homogeneous!(<T> Result<T, T> &'a mut);
impl_homogeneous!(Option<()>);
impl_homogeneous!(Option<()> &'a);
impl_homogeneous!(Option<()> &'a mut);
impl_homogeneous!(<T> ControlFlow<T, T>);
impl_homogeneous!(<T> ControlFlow<T, T> &'a);
impl_homogeneous!(<T> ControlFlow<T, T> &'a mut);

macro_rules! impl_transparent {
    ($($ty:ty)*) => {
        $(impl Homogeneous<Self> for $ty {
            #[inline]
            fn get_inner_value(self) -> Self {
                self
            }
        })*
    };
}

impl_transparent!(bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);