#[macro_export]
macro_rules! _must_unwrap {
    ($mac_call:expr) => {{
        let mut value: Option<_> = None;
        let mut early_return = true;
        let _ = (|| {
            let mut counter = 0;
            let _ = loop {
                counter += 1;
                if counter > 1 {
                    unreachable!("Macro reached continue statement");
                }
                value = Some($mac_call);
                early_return = false;
                #[allow(invalid_value)]
                return unsafe { ::core::mem::MaybeUninit::<_>::uninit().assume_init() };
            };
            #[allow(unreachable_code)]
            unreachable!("Macro reached break statement");
        })();
        assert!(!early_return, "Macro reached return statement");
        value
    }};
}

#[macro_export]
macro_rules! _must_break {
    ($mac_call:expr, $ty:ty) => {{
        let mut counter = 0;
        let break_value = loop {
            counter += 1;
            if counter > 1 {
                unreachable!("Macro reached continue statement");
            }
            let _: $ty = $mac_call;
            #[allow(unreachable_code)]
            return unreachable!("Macro did not break or continue");
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
