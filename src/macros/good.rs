use crate::Good;

#[macro_export]
macro_rules! good {
    ($enum_:expr) => {
        match Good::good($enum_) {
            Ok(v) => v,
            Err(enum_) => return enum_,
        }
    };
    ($enum_:expr; $($propagate:tt)*) => {
        match Good::good($enum_) {
            Ok(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(_) => $crate::__propagate!($($propagate)*),
        }
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        match Good::good($enum_) {
            Ok(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(enum_) => $crate::__propagate_closure!(enum_ => $($propagate_closure)*),
        }
    };
}
