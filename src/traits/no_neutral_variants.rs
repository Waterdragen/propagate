use std::ops::ControlFlow;
use crate::TwoStates;

/// Internal marker trait for enums that cannot have neutral variants.
///
/// All enums that derive [`Good`] and [`Bad`] with no neutral variants
/// implement this trait automatically
///
/// This trait is intentionally marked as unsafe for public API.
///
/// # Safety
///
/// Since enums with no neutral variants implement this trait automatically,
/// any explicit implementation of this trait makes the `TwoStates` trait fallible!
/// Implementors of this trait should always implement the [`TwoStates`] trait
/// to properly handle neutral variants.
#[allow(dead_code)]
pub unsafe trait NoNeutralVariants {}

// SAFETY: `Result` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<T, E> NoNeutralVariants for Result<T, E> {}
unsafe impl<'a, T, E> NoNeutralVariants for &'a Result<T, E> {}
unsafe impl<'a, T, E> NoNeutralVariants for &'a mut Result<T, E> {}

// SAFETY: `Option` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<T> NoNeutralVariants for Option<T> {}
unsafe impl<'a, T> NoNeutralVariants for &'a Option<T> {}
unsafe impl<'a, T> NoNeutralVariants for &'a mut Option<T> {}

// SAFETY: `ControlFlow` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<B, C> NoNeutralVariants for ControlFlow<B, C> {}
unsafe impl<'a, B, C> NoNeutralVariants for &'a ControlFlow<B, C> {}
unsafe impl<'a, B, C> NoNeutralVariants for &'a mut ControlFlow<B, C> {}

macro_rules! impl_no_neutral_variants {
    ($($ty:ty)*) => {
        $(unsafe impl NoNeutralVariants for $ty {})*
    };
}

// SAFETY: `bool` implements `Good` and `Bad`, and has exactly 2 variants
// SAFETY: primitive integers implement `Good` and `Bad`, which corresponds to
// non-zero and zero respectively, are mutually exclusive and collectively exhaustive
impl_no_neutral_variants!(bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
