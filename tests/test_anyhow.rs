#[cfg(test)]
mod tests {
    use sugar_try::{Good, good, take};
    use anyhow::{anyhow, bail, Result};

    #[derive(Debug, Good)]
    enum MyEnum {
        Zero,
        One(i32),
        #[good]
        Two(i32, i32),
        Three(i32, i32, i32),
    }
    fn test_anyhow_inner() -> Result<()> {
        let my_enum = MyEnum::Two(0, 1);
        let two: (i32, i32) = good!(my_enum => |v| bail!("Cannot get good value! The value is {:?}", v));
        assert_eq!(two, (0, 1));
        let my_enum = MyEnum::Three(0, 0, 0);
        let _: (i32, i32) = take!(my_enum, MyEnum::Two[a, b]; bail!("Cannot get two!"));
        unreachable!()
    }
    fn anyhow_eq<T: PartialEq>(res1: Result<T>, res2: Result<T>) -> bool {
        match (res1, res2) {
            (Ok(v1), Ok(v2)) => v1 == v2,
            (Err(err1), Err(err2)) => {
                err1.to_string() == err2.to_string()
            },
            _ => false,
        }
    }
    #[test]
    fn test_anyhow() {
        assert!(anyhow_eq(test_anyhow_inner(), Err(anyhow!("Cannot get two!"))));
    }
}