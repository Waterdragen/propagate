pub trait Good<T>: Sized {
    fn good(self) -> Result<T, Self>;
}
pub trait FromGood<T>: Sized {
    fn from_good(good: T) -> Self;
}
pub trait IntoGood<T> {
    fn into_good(self) -> T;
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

impl<T, U> IntoGood<U> for T where U: FromGood<T> {
    fn into_good(self) -> U {
        U::from_good(self)
    }
}
