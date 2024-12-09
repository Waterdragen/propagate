mod traits;
#[macro_use]
mod macros;
#[doc(hidden)]
pub mod __private;

pub use propagate_macros::*;
pub use macros::*;
pub use traits::{Bad, Good, NoNeutralVariants, TwoStates};
