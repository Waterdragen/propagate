#[doc(hidden)]
#[macro_export]
// Not public API. Helper macro to propagate the result for `reject!()`
macro_rules! __reject {
    ($result:expr) => {
        match $result {
            Err(v) => return v,
            Ok(enum_) => enum_,
        }
    };
    ($result:expr; $($propagate:tt)*) => {
        match $result {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(_) => $crate::__propagate!($($propagate)*),
            Ok(enum_) => enum_,
        }
    };
    ($result:expr => $($propagate_closure:tt)*) => {
        match $result {
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            Err(v) => $crate::__propagate_closure!(v => $($propagate_closure)*),
            Ok(enum_) => enum_,
        }
    };
}

#[macro_export]
macro_rules! reject {
    ($enum_:expr, $variant:path[] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant => Err(()),
            _ => Ok($enum_),
        };
        $crate::__reject!(__res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg) => Err($arg),
            _ => Ok($enum_),
        };
        $crate::__reject!(__res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident $(,$args:ident)+] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg, $($args, )+) => Err(($arg, $($args, )+)),
            _ => Ok($enum_),
        };
        $crate::__reject!(__res $($propagation)*)
    }};
}
