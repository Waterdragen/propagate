pub trait Bad<T>: Sized{
    fn bad(self) -> Result<Self, T>;
}
pub trait FromBad<T>: Sized {
    fn from_bad(bad: T) -> Self;
}

macro_rules! bad_body {
    ($generic:ident, $variant:path $(, $($borrow:tt)*)?) => {
        #[inline]
        fn bad(self) -> Result<Self, $($($borrow)*)? $generic> {
            match self {
                $variant(v) => Err(v),
                _ => Ok(self),
            }
        }
    };
}

macro_rules! impl_bad {
    (<$generic:ident $(,$second:ident)?> $generic_ty:ident $name:ident $variant:path) => {
        // Result<inner bad value, Self> (or "propagator") by ownership
        impl <$generic $(,$second)*> Bad<$generic_ty> for $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant);
        }
        // Propagator by reference
        impl <'a, $generic $(,$second)*> Bad<&'a $generic_ty> for &'a $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant, &'a);
        }
        // Propagator by mut reference
        impl <'a, $generic $(,$second)*> Bad<&'a mut $generic_ty> for &'a mut $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant, &'a mut);
        }
        // Construct owned enum
        impl <$generic $(,$second)*> FromBad<$generic_ty> for $name <$generic $(,$second)*> {
            #[inline]
            fn from_bad(bad: $generic_ty) -> Self {
                $variant(bad)
            }
        }
    };
}

use core::ops::ControlFlow;

impl_bad!(<T, E> E Result Err);
impl_bad!(<B, C> B ControlFlow ControlFlow::Break);

impl<T> Bad<()> for Option<T> {
    #[inline]
    fn bad(self) -> Result<Self, ()> {
        match self {
            None => Err(()),
            _ => Ok(self),
        }
    }
}

impl<T> FromBad<()> for Option<T> {
    #[inline]
    fn from_bad(_: ()) -> Self { None }
}

macro_rules! impl_from_bad_self {
    ($($ty:ty )*) => {
        $(impl FromBad<Self> for $ty {
            #[inline]
            fn from_bad(bad: Self) -> Self {
                bad
            }
        })*
    };
}
impl_from_bad_self!(bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! impl_bad_self {
    ($($ty:ty )*; $self:ident == $value:expr) => {
        $(impl Bad<Self> for $ty {
            #[inline]
            fn bad(self) -> Result<Self, Self> {
                if self == $value { Err(self) } else { Ok(self) }
            }
        })*
    };
}

impl_bad_self!(bool; self == false);
impl_bad_self!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize; self == 0);
