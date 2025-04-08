#[test]
fn test_texture_bindings() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/texture_binding_support.rs");
}
