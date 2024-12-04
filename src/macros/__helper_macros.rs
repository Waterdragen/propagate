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
    ($($tt:tt)*) => {{
        return $($tt)*;
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __propagate_closure {
    // Run closure then continue
    ($arg:expr => do $closure:expr; continue $($tt:tt)*) => {{
        $closure($arg);
        continue $($tt)*
    }};
    // Run closure then break
    ($arg:expr => do $closure:expr; break $($tt:tt)*) => {{
        $closure($arg);
        break $($tt)*
    }};
    // Run closure then evaluate default value
    ($arg:expr => do $closure:expr; else $($tt:tt)*) => {{
        $closure($arg);
        $($tt)*
    }};
    // Run closure then return
    ($arg:expr => do $closure:expr; $($tt:tt)*) => {{
        $closure($arg);
        return $($tt)*
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

#[macro_export]
macro_rules! __break_with_value {
    ($arg:expr; $label:lifetime $closure:expr) => { break $label ($closure)($arg) };
    ($arg:expr; $closure:expr) => { break ($closure)($arg) };
    ($($tt:tt)*) => { compile_error!("Invalid break statement") };
}
