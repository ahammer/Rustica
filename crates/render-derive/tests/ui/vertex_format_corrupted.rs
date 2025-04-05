// This test demonstrates the vertex format corruption issue by showing
// that the vertex format specified in the attribute is ignored

use rustica_render_derive::ShaderProperties;
use rustica_render::Vertex; // Make sure to import the Vertex trait
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for vertex format corruption")]
struct CorruptedFormatShader {
    #[vertex(location = 0, format = "Float32x2")]  // Should be 2D position (no Z component)
    position_2d: [f32; 2],
    
    #[vertex(location = 1, format = "Float32x3")]
    normal: [f32; 3],
    
    #[vertex(location = 2, format = "Float32x4")]
    color: [f32; 4],
}

fn main() {
    // Get the generated vertex layout
    let layout = CorruptedFormatShaderVertex::layout();
    
    // These assertions SHOULD fail because in the current implementation:
    // 1. Formats are hardcoded to Float32x3 regardless of what's specified
    // 2. Offsets are calculated incorrectly based on [f32; 3] size
    
    // Test format corruption - expects Float32x2 but gets Float32x3
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x2);
    
    // Test offset corruption
    // - First attribute offset should be 0 (this one is correct)
    assert_eq!(layout.attributes[0].offset, 0);
    
    // - Second attribute offset should be 8 bytes (after [f32; 2])
    //   But it will be 12 bytes (after [f32; 3]) due to hardcoding
    assert_eq!(layout.attributes[1].offset, 8);
    
    // - Third attribute offset should be 20 bytes (after [f32; 2] and [f32; 3])
    //   But it will be 24 bytes (after two [f32; 3]) due to hardcoding
    assert_eq!(layout.attributes[2].offset, 20);
    
    println!("This test should fail! If it succeeds, the corrupted vertex format issue is fixed.");
}