pub trait Bad<T, Propagate: ?Sized = Self> {
    fn bad(self) -> Result<Propagate, T> where Self: Sized, Propagate: Sized;
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

macro_rules! impl_option {
    ($ret:expr $(, $($borrow:tt)*)?) => {
        impl <'a, T> Bad<$($($borrow)*)? ()> for $($($borrow)*)? Option<T> {
            fn bad(self) -> Result<Self, $($($borrow)*)? ()> {
                match self {
                    None => Err($ret),
                    Some(_) => Ok(self),
                }
            }
        }
    };
}

macro_rules! impl_bad {
    (<$generic:ident $(,$second:ident)?> $generic_ty:ident $name:ident $variant:path) => {
        impl <$generic $(,$second)*> Bad<$generic_ty> for $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant);
        }
        impl <'a, $generic $(,$second)*> Bad<&'a $generic_ty> for &'a $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant, &'a);
        }
        impl <'a, $generic $(,$second)*> Bad<&'a mut $generic_ty> for &'a mut $name <$generic $(,$second)*> {
            bad_body!($generic_ty, $variant, &'a mut);
        }
    };
}

use std::ops::ControlFlow;

impl_bad!(<T, E> E Result Err);
impl_bad!(<B, C> B ControlFlow ControlFlow::Break);
impl_option!(());
impl_option!(Box::leak(Box::new(())), &'a);
impl_option!(Box::leak(Box::new(())), &'a mut);

impl Bad<Self> for bool {
    #[inline]
    fn bad(self) -> Result<Self, Self> {
        if self { Ok(self) } else { Err(self) }
    }
}

macro_rules! impl_bad_int {
    ($($ty:ty )*) => {
        $(impl Bad<Self> for $ty {
            #[inline]
            fn bad(self) -> Result<Self, Self> {
                if self != 0 { Ok(self) } else { Err(self) }
            }
        })*
    };
}

impl_bad_int!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
