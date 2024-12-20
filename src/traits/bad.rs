pub trait Bad<T>: Sized {
    fn bad(self) -> Result<Self, T>;
}
pub trait FromBad<T>: Sized {
    fn from_bad(bad: T) -> Self;
}
pub trait IntoBad<T> {
    fn into_bad(self) -> T;
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
    fn from_bad(_: ()) -> Self {
        None
    }
}

impl<T, U> IntoBad<U> for T where U: FromBad<T> {
    fn into_bad(self) -> U {
        U::from_bad(self)
    }
}
