use core::ops::ControlFlow;

/// Internal marker trait for enums that have exactly one good and one bad variant
///
/// All enums that derive [`Good`] and [`Bad`] with exactly one good and one
/// bad variant implement this trait automatically
///
/// This trait is intentionally marked as unsafe for public API.
///
/// # Safety
///
/// Since enums with exactly two variants implement this trait automatically,
/// any explicit implementation of this trait makes the `TwoStates` trait fallible!
/// Implementors of this trait should always implement the [`TwoStates`] trait
/// to properly handle neutral/duplicate good or bad variants.
#[allow(dead_code)]
pub unsafe trait ExactlyTwoDistinctVariants {}

// SAFETY: `Result` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<T, E> ExactlyTwoDistinctVariants for Result<T, E> {}
unsafe impl<'a, T, E> ExactlyTwoDistinctVariants for &'a Result<T, E> {}
unsafe impl<'a, T, E> ExactlyTwoDistinctVariants for &'a mut Result<T, E> {}

// SAFETY: `Option` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<T> ExactlyTwoDistinctVariants for Option<T> {}
unsafe impl<'a, T> ExactlyTwoDistinctVariants for &'a Option<T> {}
unsafe impl<'a, T> ExactlyTwoDistinctVariants for &'a mut Option<T> {}

// SAFETY: `ControlFlow` implements `Good` and `Bad`, and has exactly 2 variants
unsafe impl<B, C> ExactlyTwoDistinctVariants for ControlFlow<B, C> {}
unsafe impl<'a, B, C> ExactlyTwoDistinctVariants for &'a ControlFlow<B, C> {}
unsafe impl<'a, B, C> ExactlyTwoDistinctVariants for &'a mut ControlFlow<B, C> {}
