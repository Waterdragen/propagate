#[macro_export]
macro_rules! is_good {
    ($enum_:expr) => {{
        use $crate::__private::__GoodIndex;
        $enum_.__is_good()
    }};
}
