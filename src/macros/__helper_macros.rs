#[doc(hidden)]
#[macro_export]
macro_rules! __propagate {
    (continue $($tt:tt)*) => {
        continue $($tt)*
    };
    (break $($tt:tt)*) => {
        break $($tt)*
    };
    (else $($tt:tt)*) => {
        $($tt)*
    };
    (default) => {
        Default::default()
    };
    ($($tt:tt)*) => {{
        return $($tt)*;
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __propagate_closure {
    // Run closure then propagate
    ($arg:expr => do $closure:expr; $($tt:tt)*) => {{
        $closure($arg);
        $crate::__propagate!($($tt)*)
    }};

    // Catch a common mistake
    ($arg:expr => continue $($tt:tt)*) => {
        compile_error!(
        "This syntax implies \"continue with value, applying closure\", but `continue` cannot return any value.\n\
        So use `<your_enum> => do <your_closure>; continue` instead");
    };
    // Break with value, applying closure
    ($arg:expr => break $($tt:tt)*) => {
        $crate::__break_with_value!($arg; $($tt)*)
    };
    // Default value, applying closure
    ($arg:expr => else $closure:expr) => {
        ($closure)($arg)
    };
    // Return value, applying closure
    ($arg:expr => $closure:expr) => {
        return ($closure)($arg)
    };
    ($($tt:tt)*) => {
        compile_error!("Unknown syntax");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __break_with_value {
    ($arg:expr; $label:lifetime $closure:expr) => { break $label ($closure)($arg) };
    ($arg:expr; $closure:expr) => { break ($closure)($arg) };
    ($($tt:tt)*) => { compile_error!("Invalid break statement") };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __take {
    ($keep_variant:ident, $dump_variant:ident, $expr:expr) => {
        match $expr {
            $keep_variant(v) => v,
            $dump_variant(enum_) => return enum_,
        }
    };
    ($keep_variant:ident, $dump_variant:ident, $expr:expr; $($propagate:tt)*) => {
        match $expr {
            $keep_variant(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            $dump_variant(_) => $crate::__propagate!($($propagate)*),
        }
    };
    // Catch a common mistake
    ($keep_variant:ident, $dump_variant:ident, $expr:expr => full $($propagate_closure:tt)*) => {
        compile_error!("`full` can be omitted here because `take!` should be used on non-`TwoStates` enums, we can never infer the other inner value")
    };
    ($keep_variant:ident, $dump_variant:ident, $expr:expr => $($propagate_closure:tt)*) => {
        match $expr {
            $keep_variant(v) => v,
            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            $dump_variant(__enum) => $crate::__propagate_closure!(__enum => $($propagate_closure)*),
        }
    };
}
