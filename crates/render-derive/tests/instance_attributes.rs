#[test]
fn test_instance_attributes() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/instance_attribute_layout.rs");
}
