#[doc(hidden)]
#[macro_export]
// Not public API. Helper macro to propagate the result for `take!()`
macro_rules! __take {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(enum_) => return enum_,
        }
    };
    ($result:expr; $($propagate:tt)*) => {
        match $result {
            Ok(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(_) => $crate::__propagate!($($propagate)*),
        }
    };
    ($result:expr => $($propagate_closure:tt)*) => {
        match $result {
            Ok(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(enum_) => $crate::__propagate_closure!(enum_ => $($propagate_closure)*),
        }
    };
}

#[macro_export]
macro_rules! take {
    ($enum_:expr, $variant:path[] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant => Ok(()),
            _ => Err($enum_),
        };
        $crate::__take!(__res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg) => Ok($arg),
            _ => Err($enum_),
        };
        $crate::__take!(__res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident $(,$args:ident)+] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg, $($args, )+) => Ok(($arg, $($args, )+)),
            _ => Err($enum_),
        };
        $crate::__take!(__res $($propagation)*)
    }};
}
