use propagate::{good, Propagate};
mod util;
use core::ops::ControlFlow::{self, Break, Continue};

#[derive(Debug, PartialEq, Propagate)]
#[allow(dead_code)]
enum MyEnum {
    #[good]
    Zero,
    #[good]
    One(i32),
    #[good]
    OverloadOne(i32),
    #[good]
    Two(i32, i32),
    Three(i32, i32, i32),
    Named {
        id: i32,
    },
}

#[test]
fn good_value_or_return_self() {
    let opt: Option<i32> = Some(1);
    assert_unwrap_eq!(good!(opt), 1);
    let opt: Option<i32> = None;
    assert_short_circuit_eq!(good!(opt), None);

    let res: Result<i32, &str> = Ok(2);
    assert_unwrap_eq!(good!(res), 2);
    let res: Result<i32, &str> = Err("error");
    assert_short_circuit_eq!(good!(res), Err("error"));

    let control: ControlFlow<&str, i32> = Continue(3);
    assert_unwrap_eq!(good!(control), 3);
    let control: ControlFlow<&str, i32> = Break("break");
    assert_short_circuit_eq!(good!(control), Break("break"));

    let my_enum = MyEnum::Zero;
    assert_unwrap_eq!(good!(my_enum), ());
    let my_enum = MyEnum::One(1);
    assert_unwrap_eq!(good!(my_enum), 1);
    let my_enum = MyEnum::OverloadOne(1);
    assert_unwrap_eq!(good!(my_enum), 1);
    let my_enum = MyEnum::Two(2, 4);
    assert_unwrap_eq!(good!(my_enum), (2, 4));
    let my_enum = MyEnum::Three(3, 6, 9);
    assert_short_circuit_eq!(good!(my_enum), i32, MyEnum::Three(3, 6, 9));
    let my_enum = MyEnum::Zero;
    assert_short_circuit_eq!(good!(my_enum), i32, MyEnum::Zero);
}

#[test]
fn good_value_or_return_default_value() {
    let opt: Option<i32> = Some(1);
    assert_unwrap_eq!(good!(opt;), 1);
    let opt: Option<i32> = None;
    assert_short_circuit_eq!(good!(opt;), ());

    let res: Result<i32, &str> = Ok(2);
    assert_unwrap_eq!(good!(res;), 2);
    let res: Result<i32, &str> = Err("error");
    assert_short_circuit_eq!(good!(res; return), ());

    let control: ControlFlow<&str, i32> = Continue(3);
    assert_unwrap_eq!(good!(control; "default return"), 3);
    let control: ControlFlow<&str, i32> = Break("break");
    assert_short_circuit_eq!(good!(control; "default return"), "default return");
}

#[test]
fn good_value_or_use_default_value() {
    let opt: Option<i32> = Some(1);
    assert_unwrap_eq!(good!(opt; else 10), 1);
    let opt: Option<i32> = None;
    assert_unwrap_eq!(good!(opt; else 10), 10);

    let opt: Option<i32> = Some(1);
    assert_unwrap_eq!(good!(opt; default), 1);
    let opt: Option<i32> = None;
    assert_unwrap_eq!(good!(opt; default), 0);
}

#[test]
fn good_value_or_continue() {
    let res: Result<i32, &str> = Ok(2);
    assert_unwrap_eq!(good!(res; continue), 2);
    let res: Result<i32, &str> = Err("error");
    assert_continue!(good!(res; continue));
}

#[test]
fn good_value_or_break() {
    let res: Result<i32, &str> = Ok(1);
    assert_unwrap_eq!(good!(res; break), 1);
    let res: Result<i32, &str> = Err("error");
    assert_break_eq!(good!(res; break), ());

    let opt: Option<i32> = Some(2);
    assert_unwrap_eq!(good!(opt; break "break"), 2);
    let opt: Option<i32> = None;
    assert_break_eq!(good!(opt; break "break"), "break");
}

#[test]
fn good_value_or_apply_closure_not_two_states() {
    let my_enum = MyEnum::Zero;
    assert_unwrap_eq!(
        good!(my_enum => full break |enum_| matches!(enum_, MyEnum::One{..})),
        ()
    );
    let my_enum = MyEnum::One(1);
    assert_break_eq!(
        good!(my_enum => full break |enum_| matches!(enum_, MyEnum::One{..})),
        (i32, i32),
        true
    );

    let mut flag = false;
    let my_enum = MyEnum::Named { id: 0 };
    assert_short_circuit_eq!(good!(my_enum => full do |_| {flag = true;}; 4), i32, 4);
    assert!(flag);
}

#[test]
fn good_value_or_apply_closure_two_states() {
    let res: Result<i32, &str> = Err("error");
    assert_short_circuit_eq!(good!(res => |err| err), "error");
    assert_short_circuit_eq!(good!(res => _), "error");

    let res: Result<i32, &str> = Err("12");
    assert_unwrap_eq!(good!(res => else |err: &str| err.parse().unwrap()), 12);

    let res: Result<i32, &str> = Err("error");
    assert_short_circuit_eq!(good!(res => |err: &str| err.chars().next().unwrap()), 'e');

    let res: Result<i32, &str> = Err("error");
    assert_break_eq!(
        good!(res => break |err: &str| err[..3].to_owned()),
        "err".to_owned()
    );
}
