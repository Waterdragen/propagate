use std::ops::ControlFlow;
use crate::{Bad, Good, NoNeutralVariants};

// I don't know if this is needed
#[allow(dead_code)]
pub trait TwoStates<G, B>: Good<G> + Bad<B> + NoNeutralVariants {
    fn two_states(self) -> Result<G, B> where Self: Sized {
        match self.good() {
            Ok(good_value) => Ok(good_value),
            Err(self_) => {
                match self_.bad() {
                    Err(bad_value) => Err(bad_value),
                    // neutral values will reach here
                    Ok(_) => unreachable!(),
                }
            }
        }
    }
}

impl<T, E> TwoStates<T, E> for Result<T, E> {
    #[inline]
    fn two_states(self) -> Result<T, E> {
        self
    }
}
impl<'a, T, E> TwoStates<&'a T, &'a E> for &'a Result<T, E> {}
impl<'a, T, E> TwoStates<&'a mut T, &'a mut E> for &'a mut Result<T, E> {}

impl<T> TwoStates<T, ()> for Option<T> {
    #[inline]
    fn two_states(self) -> Result<T, ()> {
        self.ok_or(())
    }
}
impl<'a, T> TwoStates<&'a T, &'a ()> for &'a Option<T> {}
impl<'a, T> TwoStates<&'a mut T, &'a mut ()> for &'a mut Option<T> {}

impl<C, B> TwoStates<C, B> for ControlFlow<B, C> {}
impl<'a, C, B> TwoStates<&'a C, &'a B> for &'a ControlFlow<B, C> {}
impl<'a, C, B> TwoStates<&'a mut C, &'a mut B> for &'a mut ControlFlow<B, C> {}

macro_rules! impl_transparent {
    ($($ty:ty)*) => {
        $(impl TwoStates<Self, Self> for $ty {
            #[inline]
            fn two_states(self) -> Result<Self, Self> {
                self.good()
            }
        })*
    };
}

impl_transparent!(bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
