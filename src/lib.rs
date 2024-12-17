#![no_std]

mod traits;
#[macro_use]
mod macros;
#[doc(hidden)]
pub mod __private;

pub use propagate_macros::*;
pub use traits::*;

#[cfg(feature = "enum_index")]
pub use __private::{__GetIndex as GetIndex, __GoodIndex as GoodIndex, __BadIndex as BadIndex};
