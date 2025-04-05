#[test]
fn test_vertex_format() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/vertex_format_diagnostic.rs");
}