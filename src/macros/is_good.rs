#[macro_export]
macro_rules! is_good {
    ($enum_:expr) => {{
        use $crate::__private::__GoodIndex;
        $enum_.is_good()
    }};
}
