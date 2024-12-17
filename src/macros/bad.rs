#[macro_export]
macro_rules! bad {
    ($enum_:expr) => {
        $crate::__take!(Err, Ok, $crate::Bad::bad($enum_))
    };
    ($enum_:expr; $($propagate:tt)*) => {
        $crate::__take!(Err, Ok, $crate::Bad::bad($enum_); $($propagate)*)
    };
    ($enum_:expr => full $($propagate_closure:tt)*) => {
        $crate::__take!(Err, Ok, $crate::Bad::bad($enum_) => $($propagate_closure)*)
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        $crate::__take!(Err, Ok, $crate::TwoStates::two_states($enum_) => $($propagate_closure)*)
    };
}
