use propagate::{bad, Propagate};
mod util;
use core::ops::ControlFlow::{self, Break, Continue};

#[derive(Debug, PartialEq, Propagate)]
#[allow(dead_code)]
enum MyEnum {
    #[bad]
    Zero,
    #[bad]
    One(i32),
    #[bad]
    OverloadOne(i32),
    #[bad]
    Two(i32, i32),
    Three(i32, i32, i32),
    Named {
        id: i32,
    },
}

#[test]
fn bad_value_or_return_self() {
    let opt: Option<i32> = Some(1);
    assert_short_circuit_eq!(bad!(opt), Some(1));
    let opt: Option<i32> = None;
    assert_unwrap_eq!(bad!(opt), ());

    let res: Result<i32, &str> = Ok(2);
    assert_short_circuit_eq!(bad!(res), Ok(2));
    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res), "error");

    let control: ControlFlow<&str, i32> = Continue(3);
    assert_short_circuit_eq!(bad!(control), Continue(3));
    let control: ControlFlow<&str, i32> = Break("break");
    assert_unwrap_eq!(bad!(control), "break");

    let my_enum = MyEnum::Zero;
    assert_unwrap_eq!(bad!(my_enum), ());
    let my_enum = MyEnum::One(1);
    assert_unwrap_eq!(bad!(my_enum), 1);
    let my_enum = MyEnum::OverloadOne(1);
    assert_unwrap_eq!(bad!(my_enum), 1);
    let my_enum = MyEnum::Two(2, 4);
    assert_unwrap_eq!(bad!(my_enum), (2, 4));
    let my_enum = MyEnum::Three(3, 6, 9);
    assert_short_circuit_eq!(bad!(my_enum), i32, MyEnum::Three(3, 6, 9));
    let my_enum = MyEnum::Zero;
    assert_short_circuit_eq!(bad!(my_enum), i32, MyEnum::Zero);
}

#[test]
fn bad_value_or_return_default_value() {
    let opt: Option<i32> = Some(1);
    assert_short_circuit_eq!(bad!(opt;), ());
    let opt: Option<i32> = None;
    assert_unwrap_eq!(bad!(opt; 2), ());

    let res: Result<i32, &str> = Ok(2);
    assert_short_circuit_eq!(bad!(res;), ());
    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res; return), "error");

    let control: ControlFlow<&str, i32> = Continue(3);
    assert_short_circuit_eq!(bad!(control; "default return"), "default return");
    let control: ControlFlow<&str, i32> = Break("break");
    assert_unwrap_eq!(bad!(control; "default return"), "break");
}

#[test]
fn bad_value_or_use_default_value() {
    let res: Result<i32, i32> = Ok(1);
    assert_unwrap_eq!(bad!(res; else 2), 2);
    let res: Result<i32, i32> = Err(1);
    assert_unwrap_eq!(bad!(res; else 10), 1);

    let res: Result<i32, i32> = Ok(1);
    assert_unwrap_eq!(bad!(res; default), 0);
    let res: Result<i32, i32> = Err(10);
    assert_unwrap_eq!(bad!(res; default), 10);
}

#[test]
fn bad_value_or_continue() {
    let res: Result<i32, &str> = Ok(2);
    assert_continue!(bad!(res; continue));
    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res; continue), "error");
}

#[test]
fn bad_value_or_break() {
    let res: Result<i32, &str> = Ok(1);
    assert_break_eq!(bad!(res; break), ());
    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res; break), "error");

    let opt: Option<i32> = Some(2);
    assert_break_eq!(bad!(opt; break "break"), "break");
    let opt: Option<i32> = None;
    assert_unwrap_eq!(bad!(opt; break "break"), ());
}

#[test]
fn bad_value_or_apply_closure_not_two_states() {
    let my_enum = MyEnum::Zero;
    assert_unwrap_eq!(
        bad!(my_enum => full break |enum_| matches!(enum_, MyEnum::One{..})),
        ()
    );
    let my_enum = MyEnum::One(1);
    assert_break_eq!(
        bad!(my_enum => full break |enum_| matches!(enum_, MyEnum::One{..})),
        (i32, i32),
        true
    );

    let mut flag = false;
    let my_enum = MyEnum::Named { id: 0 };
    assert_short_circuit_eq!(bad!(my_enum => full do |_| {flag = true;}; 4), i32, 4);
    assert!(flag);
}

#[test]
fn bad_value_or_apply_closure_two_states() {
    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res => |err| err), "error");
    assert_unwrap_eq!(bad!(res => _), "error");

    let res: Result<i32, &str> = Err("12");
    assert_unwrap_eq!(
        bad!(res => else |ok: i32| Box::leak((ok + 1).to_string().into_boxed_str())),
        "12"
    );

    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res => |ok: i32| ok / 2), "error");

    let res: Result<i32, &str> = Err("error");
    assert_unwrap_eq!(bad!(res => break |ok: i32| ok * 2), "error");
}
