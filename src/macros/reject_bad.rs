use crate::Bad;

#[macro_export]
macro_rules! reject_bad {
    ($enum_:expr) => {
        match Bad::bad($enum_) {
            Ok(v) => return v,
            Err(enum_) => enum_,
        }
    };
    ($enum_:expr; $($propagate:tt)*) => {
        match $crate::is_bad!($enum_) {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            true => $crate::__propagate!($($propagate)*),
            false => $enum_,
        }
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        match Bad::bad($enum_) {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(v) => $crate::__propagate_closure!(v => $($propagate_closure)*),
            Ok(enum_) => enum_,
        }
    };
}
