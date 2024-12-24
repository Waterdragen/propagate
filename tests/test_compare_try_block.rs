#![feature(try_blocks)]

use propagate::{good, Bad, Good};
use std::num::ParseIntError;

fn add_try_blocks(str1: &str, str2: &str) -> i8 {
    let result: Result<_, ParseIntError> = try {
        let int1 = str1.parse::<i8>();
        let int2 = str2.parse::<i8>();
        int1? + int2?
    };
    result.unwrap_or(-1)
}

fn add_propagate(str1: &str, str2: &str) -> i8 {
    let int1 = good!(str1.parse::<i8>(); -1);
    let int2 = good!(str2.parse::<i8>(); -1);
    int1 + int2
}

#[test]
fn compare_add() {
    let a = "12";
    let b = "21";
    assert_eq!(add_try_blocks(a, b), 33);
    assert_eq!(add_propagate(a, b), 33);

    let c = "abc";
    let d = "123";
    assert_eq!(add_try_blocks(c, d), -1);
    assert_eq!(add_propagate(c, d), -1);
}

fn multiple_results_try_block(
    stderr: &mut String,
    res1: Result<i32, i32>,
    res2: Result<i32, i32>,
    res3: Result<i32, i32>,
) {
    let res: Result<_, i32> = try {
        res1?;
        res2?;
        res3?;
    };
    if let Err(_err) = res {
        stderr.push_str("failure");
    }
}

fn multiple_results_propagate(
    stderr: &mut String,
    res1: Result<i32, i32>,
    res2: Result<i32, i32>,
    res3: Result<i32, i32>,
) {
    let res: Result<_, i32> = (|| {
        good!(res1 => Bad);
        good!(res2 => Bad);
        good!(res3 => Bad);
        Good(())
    })();
    if let Err(_err) = res {
        stderr.push_str("failure");
    }
}

#[test]
fn compare_multiple_results() {
    let mut stderr = String::new();
    let res1: Result<i32, i32> = Ok(1);
    let res2: Result<i32, i32> = Ok(2);
    let res3: Result<i32, i32> = Err(3);

    multiple_results_try_block(&mut stderr, res1, res2, res3);
    assert_eq!(stderr, "failure");

    let mut stderr = String::new();
    let res1: Result<i32, i32> = Ok(1);
    let res2: Result<i32, i32> = Ok(2);
    let res3: Result<i32, i32> = Err(3);

    multiple_results_propagate(&mut stderr, res1, res2, res3);
    assert_eq!(stderr, "failure");
}
