#[test]
fn test_duplicate_param_fails() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/dx/*.rs");
}
