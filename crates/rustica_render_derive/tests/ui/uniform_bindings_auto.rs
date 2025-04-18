// Test for automatic uniform binding assignment
// This test verifies that uniform bindings are correctly auto-assigned
// and that explicitly set bindings are respected

use rustica_render_derive::ShaderProperties;
use rustica_foundation::Vertex;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for uniform bindings")]
struct UniformBindingsShader {
    // Auto-assigned binding (should get binding 0)
    #[uniform]
    model: [[f32; 4]; 4],
    
    // Explicitly set binding 2 (skipping 1)
    #[uniform(binding = 2)]
    view: [[f32; 4]; 4],
    
    // Auto-assigned binding (should get binding 1, filling the gap)
    #[uniform]
    projection: [[f32; 4]; 4],
    
    // No conflicts with vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    normal: [f32; 3],
}

fn main() {
    // Get the shader descriptor
    let descriptor = UniformBindingsShader::descriptor();
    
    // Verify number of uniforms
    assert_eq!(descriptor.uniforms.len(), 3);
    
    // Check uniform names and bindings
    let mut found_model = false;
    let mut found_view = false;
    let mut found_projection = false;
    
    for uniform in &descriptor.uniforms {
        match uniform.name.as_str() {
            "model" => {
                found_model = true;
                assert_eq!(uniform.binding, 0, "model should have binding 0");
            },
            "view" => {
                found_view = true;
                assert_eq!(uniform.binding, 2, "view should have binding 2");
            },
            "projection" => {
                found_projection = true;
                assert_eq!(uniform.binding, 1, "projection should have binding 1");
            },
            _ => panic!("Unexpected uniform name: {}", uniform.name),
        }
    }
    
    assert!(found_model, "model uniform not found");
    assert!(found_view, "view uniform not found");
    assert!(found_projection, "projection uniform not found");
    
    println!("Uniform binding auto-assignment test passed!");
}
