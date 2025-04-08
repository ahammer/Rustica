#[test]
fn test_duplicate_locations() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_location_error.rs");
}
