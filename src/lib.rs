#![no_std]

mod traits;
#[macro_use]
mod macros;
#[doc(hidden)]
pub mod __private;

pub use propagate_derive::*;
pub use traits::*;

#[cfg(feature = "enum_index")]
pub use __private::{__BadIndex as BadIndex, __GetIndex as GetIndex, __GoodIndex as GoodIndex};
