pub trait Good<T>: Sized {
    fn good(self) -> Result<T, Self>;
}
pub trait FromGood<T>: Sized {
    fn from_good(good: T) -> Self;
}

macro_rules! good_body {
    ($generic:ident, $variant:path $(, $($borrow:tt)*)?) => {
        #[inline]
        fn good(self) -> Result<$($($borrow)*)? $generic, Self> {
            match self {
                $variant(v) => Ok(v),
                _ => Err(self),
            }
        }
    };
}

macro_rules! impl_good {
    (<$generic:ident $(,$second:ident)?> $generic_ty:ident $name:ident $variant:path) => {
        // Result<inner good value, Self> (or "propagator") by ownership
        impl <$generic $(,$second)*> Good<$generic_ty> for $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant);
        }
        // Propagator by reference
        impl <'a, $generic $(,$second)*> Good<&'a $generic_ty> for &'a $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant, &'a);
        }
        // Propagator by mut reference
        impl <'a, $generic $(,$second)*> Good<&'a mut $generic_ty> for &'a mut $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant, &'a mut);
        }
        // Construct owned enum
        impl <$generic $(,$second)*> FromGood<$generic_ty> for $name <$generic $(,$second)*> {
            #[inline]
            fn from_good(good: $generic_ty) -> Self {
                $variant(good)
            }
        }
    };
}

use core::ops::ControlFlow;

impl_good!(<T, E> T Result Ok);
impl_good!(<T> T Option Some);
impl_good!(<B, C> C ControlFlow ControlFlow::Continue);

macro_rules! impl_good_self {
    ($($ty:ty )*; $self:ident != $value:expr) => {
        $(impl Good<Self> for $ty {
            #[inline]
            fn good(self) -> Result<Self, Self> {
                if self != $value { Ok(self) } else { Err(self) }
            }
        })*
    };
}

impl_good_self!(bool; self != false);
impl_good_self!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize; self != 0);

macro_rules! impl_from_good_self {
    ($($ty:ty )*) => {
        $(impl FromGood<Self> for $ty {
            #[inline]
            fn from_good(good: Self) -> Self { good }
        })*
    };
}
impl_from_good_self!(bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
