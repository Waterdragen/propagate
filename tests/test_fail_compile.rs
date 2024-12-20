#[test]
fn fail_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail_compile/*.rs");
}
