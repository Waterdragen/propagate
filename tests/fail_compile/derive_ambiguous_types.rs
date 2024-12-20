use propagate::Propagate;

#[derive(Propagate)]
enum MyEnum {
    #[good]
    A(i32, i32),
    #[good]
    B((i32, i32)),
}

fn main() {}
