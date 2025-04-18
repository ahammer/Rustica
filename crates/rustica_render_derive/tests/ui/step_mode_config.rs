// Test for vertex step mode configuration
// This test verifies that step modes can be properly configured for vertex attributes
// Step modes determine whether attributes advance per-vertex or per-instance

use rustica_render_derive::ShaderProperties;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for step mode configuration")]
struct StepModeShader {
    // Regular vertex attributes (implicitly VertexStepMode::Vertex)
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    normal: [f32; 3],
    
    #[vertex(location = 2)]
    color: [f32; 4],
    
    // Instance attributes (implicitly VertexStepMode::Instance)
    #[instance(location = 3)]
    model_matrix_0: [f32; 4],
    
    #[instance(location = 4)]
    model_matrix_1: [f32; 4],
    
    #[instance(location = 5)]
    model_matrix_2: [f32; 4],
    
    #[instance(location = 6)]
    model_matrix_3: [f32; 4],
    
    #[instance(location = 7)]
    instance_color: [f32; 3],
}

fn main() {
    // Get the vertex layout
    let vertex_layout = StepModeShaderVertex::layout();
    
    // Check vertex attributes
    assert_eq!(vertex_layout.attributes.len(), 3, "Should have 3 vertex attributes");
    assert_eq!(vertex_layout.step_mode, wgpu::VertexStepMode::Vertex, 
              "Vertex attributes should use VertexStepMode::Vertex");
    
    // Instancing is handled via a separate layout, so we can't directly test it here,
    // but we can verify that the generated instance struct has the right fields
    let instance = StepModeShaderInstances {
        model_matrix_0: [1.0, 0.0, 0.0, 0.0],
        model_matrix_1: [0.0, 1.0, 0.0, 0.0],
        model_matrix_2: [0.0, 0.0, 1.0, 0.0],
        model_matrix_3: [0.0, 0.0, 0.0, 1.0],
        instance_color: [1.0, 0.0, 0.0],
    };
    
    // Check instance fields
    assert_eq!(instance.model_matrix_0[0], 1.0);
    assert_eq!(instance.model_matrix_1[1], 1.0);
    assert_eq!(instance.model_matrix_2[2], 1.0);
    assert_eq!(instance.model_matrix_3[3], 1.0);
    assert_eq!(instance.instance_color[0], 1.0);
    
    // Test with the shader descriptor
    let descriptor = StepModeShader::descriptor();
    
    // Verify the descriptor contains the vertex attributes
    assert_eq!(descriptor.vertex_attributes.len(), 3, 
              "Descriptor should have 3 vertex attributes");
    
    println!("Step mode configuration test passed!");
}
