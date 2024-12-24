#![no_std]

mod traits;
#[macro_use]
mod macros;
#[doc(hidden)]
pub mod __private;

pub use propagate_derive::*;
pub use traits::*;

#[allow(non_snake_case)]
pub fn Good<Target, G>(value: G) -> Target
where
    Target: FromGood<G>,
{
    FromGood::from_good(value)
}

#[allow(non_snake_case)]
pub fn Bad<Target, B>(value: B) -> Target
where
    Target: FromBad<B>,
{
    FromBad::from_bad(value)
}

#[cfg(feature = "enum_index")]
pub use __private::{__BadIndex as BadIndex, __GetIndex as GetIndex, __GoodIndex as GoodIndex};
