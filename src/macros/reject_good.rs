use crate::Good;

#[macro_export]
macro_rules! reject_good {
    ($enum_:expr) => {
        match Good::good($enum_) {
            Ok(v) => return v,
            Err(enum_) => enum_,
        }
    };
    ($enum_:expr; $($propagate:tt)*) => {
        match $crate::is_good!($enum_) {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            true => $crate::__propagate!($($propagate)*),
            false => $enum_,
        }
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        match Good::good($enum_) {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Ok(v) => $crate::__propagate_closure!(v => $($propagate_closure)*),
            Err(enum_) => enum_,
        }
    };
}
