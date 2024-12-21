#[macro_export]
macro_rules! _must_unwrap {
    ($mac_call:expr) => {{
        use core::ops::ControlFlow::{self, Break, Continue};
        use std::sync::OnceLock;
        let control: OnceLock<ControlFlow<()>> = OnceLock::new();
        let value: OnceLock<_> = OnceLock::new();
        #[allow(unreachable_code)]
        #[allow(clippy::diverging_sub_expression)]
        let res = std::panic::catch_unwind(|| {
            let _ = (|| {
                let mut counter = 0;
                let _ = loop {
                    counter += 1;
                    if counter > 1 {
                        let _ = control.set(Continue(()));
                        panic!();
                    }
                    let _ = value.set($mac_call);
                    return panic!();
                };
                let _ = control.set(Break(()));
                panic!();
            })();
        });
        assert!(res.is_err(), "Macro reached return statement");
        match control.into_inner() {
            Some(Continue(_)) => panic!("Macro reached continue statement"),
            Some(Break(_)) => panic!("Macro reached break statement"),
            None => {}
        }
        value.into_inner()
    }};
}

#[macro_export]
macro_rules! _must_break {
    ($mac_call:expr, $ty:ty) => {{
        let mut counter = 0;
        #[allow(unreachable_code)]
        #[allow(clippy::diverging_sub_expression)]
        let break_value = loop {
            counter += 1;
            if counter > 1 {
                unreachable!("Macro reached continue statement");
            }
            let _: $ty = $mac_call;
            break unreachable!("Macro did not break or continue");
        };
        break_value
    }};
    ($mac_call:expr) => {
        _must_break!($mac_call, _)
    };
}

/// Asserts the macro should give the inner value,
/// and value should equal right hand side
#[macro_export]
macro_rules! assert_unwrap_eq {
    ($mac_call:expr, $good_expr:expr) => {
        assert_eq!(
            $crate::_must_unwrap!($mac_call),
            Some($good_expr),
            "Macro short circuited"
        )
    };
}

/// Asserts the macro should short circuit,
/// and the returned value should equal right hand side
#[macro_export]
macro_rules! assert_short_circuit_eq {
    ($mac_call:expr, $return_expr:expr) => {
        assert_short_circuit_eq!($mac_call, _, $return_expr)
    };
    ($mac_call:expr, $ty:ty, $bad_expr:expr) => {{
        assert_eq!(
            (|| {
                let _: $ty = $mac_call;
                unreachable!("Macro did not short circuit");
            })(),
            $bad_expr
        );
    }};
}

/// Asserts the macro should continue
#[macro_export]
macro_rules! assert_continue {
    ($mac_call:expr, $ty:ty) => {{
        use core::ops::ControlFlow::{self, Break, Continue};
        let mut control: ControlFlow<()> = Break(());
        for i in (0..=1) {
            if i == 1 {
                control = Continue(());
                break;
            }
            let _: $ty = $mac_call;
            unreachable!("Macro did not continue");
        }
        assert_eq!(control, Continue(()));
    }};
    ($mac_call:expr) => {
        assert_continue!($mac_call, _)
    };
}

/// Asserts the macro should break
#[macro_export]
macro_rules! assert_break_eq {
    ($mac_call:expr, $ty:ty, $break_expr:expr) => {
        assert_eq!(_must_break!($mac_call, $ty), $break_expr);
    };
    ($mac_call:expr, $break_expr:expr) => {
        assert_break_eq!($mac_call, _, $break_expr)
    };
}
