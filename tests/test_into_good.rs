use core::ops::ControlFlow::{self, Continue};
use propagate::{good, IntoGood, Propagate};

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
fn into_good() {
    let opt: Option<_> = 1.into_good();
    assert_eq!(opt, Some(1));

    let res: Result<i32, i32> = 2.into_good();
    assert_eq!(res, Ok(2));

    let control: ControlFlow<i32, i32> = 3.into_good();
    assert_eq!(control, Continue(3));

    let my_enum: MyEnum = (4, 4).into_good();
    assert_eq!(my_enum, MyEnum::Two(4, 4));
}
