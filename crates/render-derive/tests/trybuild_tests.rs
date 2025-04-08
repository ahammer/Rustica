// Consolidated trybuild tests for render-derive
// This file contains all the tests for the ShaderProperties derive macro

// Test for vertex format features
#[test]
fn test_vertex_format_diagnostic() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/vertex_format_diagnostic.rs");
}

// Test for vertex format corruption detection
#[test]
fn test_vertex_format_corrupted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/vertex_format_corrupted.rs");
}

// Test for uniform binding auto-assignment
#[test]
fn test_uniform_bindings() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/uniform_bindings_auto.rs");
}

// Test for duplicate location detection
#[test]
fn test_duplicate_locations() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_location_error.rs");
}

// Test for instance attribute layout
#[test]
fn test_instance_attributes() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/instance_attribute_layout.rs");
}

// Test for texture binding support
#[test]
fn test_texture_bindings() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/texture_binding_support.rs");
}
