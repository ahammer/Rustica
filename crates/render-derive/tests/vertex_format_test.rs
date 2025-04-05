#[test]
fn test_vertex_format() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/vertex_format_corrupted.rs");
}