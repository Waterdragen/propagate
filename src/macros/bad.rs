use crate::Bad;

#[macro_export]
macro_rules! bad {
    ($enum_:expr) => {
        match Bad::bad($enum_) {
            Err(v) => v,
            Ok(enum_) => return enum_,
        }
    };
    ($enum_:expr; $($propagate:tt)*) => {
        match Bad::bad($enum_) {
            Err(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Ok(_) => $crate::__propagate!($($propagate)*),
        }
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        match Bad::bad($enum_) {
            Err(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Ok(enum_) => $crate::__propagate_closure!(enum_ => $($propagate_closure)*),
        }
    };
}
