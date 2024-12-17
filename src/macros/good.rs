#[macro_export]
macro_rules! good {
    ($enum_:expr) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_))
    };
    ($enum_:expr; $($propagate:tt)*) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_); $($propagate)*)
    };
    ($enum_:expr => full $($propagate_closure:tt)*) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_) => $($propagate_closure)*)
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        $crate::__take!(Ok, Err, $crate::TwoStates::two_states($enum_) => $($propagate_closure)*)
    };
}
