pub trait Good<T, Propagate: ?Sized = Self> {
    fn good(self) -> Result<T, Propagate> where Self: Sized, Propagate: Sized;
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
        impl <$generic $(,$second)*> Good<$generic_ty> for $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant);
        }
        impl <'a, $generic $(,$second)*> Good<&'a $generic_ty> for &'a $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant, &'a);
        }
        impl <'a, $generic $(,$second)*> Good<&'a mut $generic_ty> for &'a mut $name <$generic $(,$second)*> {
            good_body!($generic_ty, $variant, &'a mut);
        }
    };
}

use std::ops::ControlFlow;

impl_good!(<T, E> T Result Ok);
impl_good!(<T> T Option Some);
impl_good!(<B, C> C ControlFlow ControlFlow::Continue);

impl Good<Self> for bool {
    #[inline]
    fn good(self) -> Result<Self, Self> {
        if self { Ok(self) } else { Err(self) }
    }
}

macro_rules! impl_good_int {
    ($($ty:ty )*) => {
        $(impl Good<Self> for $ty {
            #[inline]
            fn good(self) -> Result<Self, Self> {
                if self != 0 { Ok(self) } else { Err(self) }
            }
        })*
    };
}

impl_good_int!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
