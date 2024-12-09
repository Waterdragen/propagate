#[macro_export]
macro_rules! take {
    ($enum_:expr, $variant:path[] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant => Ok(()),
            _ => Err($enum_),
        };
        $crate::__take!(Ok, Err, __res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg) => Ok($arg),
            _ => Err($enum_),
        };
        $crate::__take!(Ok, Err, __res $($propagation)*)
    }};
    ($enum_:expr, $variant:path[$arg:ident $(,$args:ident)+] $($propagation:tt)*) => {{
        let __res = match $enum_ {
            $variant($arg, $($args, )+) => Ok(($arg, $($args, )+)),
            _ => Err($enum_),
        };
        $crate::__take!(Ok, Err, __res $($propagation)*)
    }};
}
