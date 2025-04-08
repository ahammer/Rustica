#[test]
fn test_uniform_bindings() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/uniform_bindings_auto.rs");
}
