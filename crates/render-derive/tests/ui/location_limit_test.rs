// Test for shader location limit validation
// This test verifies the macro properly handles WGPU's shader location limits
// WGPU has a limit on the number of vertex attributes per shader (typically 16)

use rustica_render_derive::ShaderProperties;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for location limits")]
struct LocationLimitShader {
    // Add the maximum reasonable number of vertex attributes to test location limit handling
    #[vertex(location = 0)]
    attr0: [f32; 4],
    
    #[vertex(location = 1)]
    attr1: [f32; 4],
    
    #[vertex(location = 2)]
    attr2: [f32; 4],
    
    #[vertex(location = 3)]
    attr3: [f32; 4],
    
    #[vertex(location = 4)]
    attr4: [f32; 4],
    
    #[vertex(location = 5)]
    attr5: [f32; 4],
    
    #[vertex(location = 6)]
    attr6: [f32; 4],
    
    #[vertex(location = 7)]
    attr7: [f32; 4],
    
    #[vertex(location = 8)]
    attr8: [f32; 4],
    
    #[vertex(location = 9)]
    attr9: [f32; 4],
    
    #[vertex(location = 10)]
    attr10: [f32; 4],
    
    #[vertex(location = 11)]
    attr11: [f32; 4],
    
    #[vertex(location = 12)]
    attr12: [f32; 4],
    
    #[vertex(location = 13)]
    attr13: [f32; 4],
    
    #[vertex(location = 14)]
    attr14: [f32; 4],
    
    #[vertex(location = 15)]
    attr15: [f32; 4],
}

fn main() {
    // Get the vertex layout
    let layout = LocationLimitShaderVertex::layout();
    
    // Check total number of attributes
    assert_eq!(layout.attributes.len(), 16, "Should have 16 attributes");
    
    // Check that shader locations are assigned sequentially
    for i in 0..16 {
        assert_eq!(layout.attributes[i].shader_location, i as u32,
                  "Attribute {i} should have location {i}");
    }
    
    // Check the step mode (should be Vertex)
    assert_eq!(layout.step_mode, wgpu::VertexStepMode::Vertex);
    
    println!("Shader location limit test passed!");
}
