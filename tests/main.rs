use std::error::Error;
use sugar_try::{bad, good, is_good, reject, reject_bad, reject_good, take, Bad, Good};

#[derive(Debug, Good, PartialEq)]
enum MyEnum {
    #[good]
    Zero,
    #[good]
    One(i32),
    #[good]
    Overload(i32),
    #[good]
    Two(i32, i32),
    Three(i32, i32, i32),
    Named {
        id: i32,
    },
}

#[derive(Debug, Good, Bad)]
enum MySwitch {
    #[good]
    High,
    #[bad]
    Low,
}

#[derive(Debug, Good, Bad, PartialEq)]
enum LogData {
    #[good] SuccessMsg(String),
            InfoMsg(String),
            DebugMsg(String),
    #[bad]  ErrorCode(u32),
    #[bad]  ErrorMsg(String),
}

fn append_party_emoji_on_success(log_data: LogData) -> LogData {
    // We can infer type here because we only have one #[good] variant
    let mut success_msg = good!(log_data);
    success_msg.push_str(" 🎉🎉🎉");
    LogData::SuccessMsg(success_msg)
}

fn get_success_msg_len_or_format_str(log_data: &LogData) -> Result<usize, String> {
    let success_msg = good!(log_data => |data| Err(format!("{:?}", data)));
    Ok(success_msg.len())
}

fn reset_error_code_on_error(log_data: &mut LogData) {
    let error_code: &mut u32 = bad!(log_data;);  // Must annotate type here
    // let error_code = bad!(&mut log_data; ());
    // ^^^^ FAIL: Cannot infer type because we have two #[bad] variants!
    *error_code = 0;
}

fn main() {
    let log_data = LogData::SuccessMsg("This is a success message".to_owned());
    let log_data = append_party_emoji_on_success(log_data);
    assert_eq!(
        log_data,
        LogData::SuccessMsg("This is a success message 🎉🎉🎉".to_owned())
    );

    let log_data = LogData::ErrorMsg("This is an error message".to_owned());
    let log_data = append_party_emoji_on_success(log_data);
    assert_eq!(
        log_data,
        LogData::ErrorMsg("This is an error message".to_owned())
    );

    let mut log_data = LogData::ErrorCode(2);
    reset_error_code_on_error(&mut log_data);
    assert_eq!(log_data, LogData::ErrorCode(0));

    let log_data = LogData::InfoMsg("This is an info message".to_owned());
    let res = get_success_msg_len_or_format_str(&log_data);
    assert_ne!(res, Ok(23));
    assert_eq!(res, Err(r#"InfoMsg("This is an info message")"#.to_owned()));

    let mut v: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3), Err("four"), Ok(5)];
    'a: for item in v.iter_mut() {
        let item: &mut i32 = good!(item; continue 'a);
        *item *= 2;
    }
    let value = 'a: loop {
        let option: Option<i32> = None;
        good!(option; break 'a 123);
    };
    assert_eq!(value, 123);
    println!("Returned value from loop: {:?}", value);
    // assert_eq!(v, vec![Ok(2), Ok(4), Ok(6), Err("four"), Ok(5)]);
    assert_eq!(v, vec![Ok(2), Ok(4), Ok(6), Err("four"), Ok(10)]);

    let _my_enum = MyEnum::Two(1, 2);
    // let value = take!(&_my_enum, MyEnum::Zero:(); ());
    // unreachable!();
    // let value = take!(&_my_enum, MyEnum::One:(_v); ());
    // unreachable!();

    // let _my_enum = MyEnum::Zero;
    // let _value = take!(&_my_enum, MyEnum::Two:(a, b););
    // unreachable!();

    let mut my_enum = MyEnum::Zero;
    assert!(is_good!(&my_enum));
    assert!(is_good!(&mut my_enum));
    let my_enum = MyEnum::Three(1, 2, 3);
    let _value: &MyEnum = reject_good!(&my_enum;);
    let _value: MyEnum = reject_good!(my_enum;);
    println!("Reachable code");
    // propagate_closure_test!(my_enum; |v| 1 => continue);

    let items = [Ok(1), Ok(2), Err("three"), Ok(4), Ok(5)];
    let mut new_vec: Vec<i32> = Vec::new();
    let mut err_item: Option<&Result<i32, &str>> = None;
    for item in items.iter() {
        let num = good!(item => do |item| {err_item = Some(item)}; continue);
        new_vec.push(*num);
    }
    assert_eq!(new_vec, [1, 2, 4, 5]);
    assert_eq!(err_item, Some(&Err("three")));

    new_vec.clear();
    err_item = None;
    for item in items.iter() {
        let num = good!(item => do |item| {err_item = Some(item)}; break);
        new_vec.push(*num);
    }
    assert_eq!(new_vec, [1, 2]);
    assert_eq!(err_item, Some(&Err("three")));

    let mut good_value: Option<i32> = None;
    let got_a_good_value = 'a: loop {
        let i: Result<i32, &str> = Err("an error");
        let inner_value = good!(i => break 'a |v: Result<i32, &str>| v.is_ok());
        good_value = Some(inner_value);
    };
    assert!(!got_a_good_value);
    assert_eq!(good_value, None);

    println!("Reachable code");

    use anyhow::{bail};
    fn test_anyhow_inner() -> anyhow::Result<()> {
        let my_enum = MyEnum::Two(0, 1);
        let two: (i32, i32) = good!(my_enum; bail!("Cannot get good value!"));
        assert_eq!(two, (0, 1));

        let my_res: Result<i32, &str> = Err("");
        let num = good!(my_res => |v| bail!("{:?}", v));
        assert_eq!(num, 5);

        let my_enum = MyEnum::Three(0, 0, 0);
        let _: (i32, i32) = take!(my_enum, MyEnum::Two[a, b]; bail!("Cannot get two!"));
        unreachable!()
    }
    println!("{:?}", test_anyhow_inner());

    fn test_reject() {
        let a: Result<i32, i32> = Err(32);
        let b = reject!(a, Err[v] => else Err);
        assert_eq!(b, Err(32));
    }
    test_reject();

    let ref_option_t = &Some(8);
    let option_ref_t = Some(&8);
    let a: &i32 = good!(ref_option_t; ());
    let b: &i32 = good!(&option_ref_t;);

    #[allow(clippy::never_loop)]
    let _never = loop {
        let res: Result<&str, &str> = Ok("always ok");
        bad!(res => |_| {println!("Before returning in loop");});
        unreachable!()
    };
}
