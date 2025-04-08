// Test for array stride alignment
// This test verifies that the shader properties macro correctly
// calculates array stride with proper Vulkan/WGPU alignment rules
// for various data types with different sizes and potential padding

use rustica_render_derive::ShaderProperties;
use wgpu;

// Struct with fields of different sizes to test alignment and stride calculation
#[derive(ShaderProperties)]
#[shader(inline = "test shader for array stride alignment")]
struct AlignmentTestShader {
    // Vertex attributes with different sizes
    #[vertex(location = 0)]
    position: [f32; 3],     // 12 bytes
    
    #[vertex(location = 1)]
    normal: [f32; 3],       // 12 bytes
    
    #[vertex(location = 2)]
    tex_coord: [f32; 2],    // 8 bytes
    
    #[vertex(location = 3)]
    color: [f32; 4],        // 16 bytes
    
    // Include a scalar value to test potential padding
    #[vertex(location = 4)]
    alpha: f32,             // 4 bytes

    // Include a non-aligned attribute
    #[vertex(location = 5)]
    bone_indices: [u8; 4],  // 4 bytes (potentially could need alignment)
    
    // Include a larger type that might need alignment
    #[vertex(location = 6)]
    tangent: [f32; 4],      // 16 bytes
}

fn main() {
    // Get the vertex layout
    let layout = AlignmentTestShaderVertex::layout();
    
    // Check total size of the vertex struct
    let vertex_size = std::mem::size_of::<AlignmentTestShaderVertex>();
    println!("Vertex struct size: {} bytes", vertex_size);
    
    // The array_stride should match the total vertex size
    assert_eq!(layout.array_stride as usize, vertex_size, 
               "Array stride should match the size of the vertex struct");
    
    // Check that shader locations are assigned as expected
    assert_eq!(layout.attributes[0].shader_location, 0);
    assert_eq!(layout.attributes[1].shader_location, 1);
    assert_eq!(layout.attributes[2].shader_location, 2);
    assert_eq!(layout.attributes[3].shader_location, 3);
    assert_eq!(layout.attributes[4].shader_location, 4);
    assert_eq!(layout.attributes[5].shader_location, 5);
    assert_eq!(layout.attributes[6].shader_location, 6);
    
    // Check that the offsets are correct (depends on data layout in memory)
    // For a tightly packed struct, we expect:
    // position: offset = 0
    assert_eq!(layout.attributes[0].offset, 0);
    
    // normal: offset = position size
    assert_eq!(layout.attributes[1].offset, 12);
    
    // tex_coord: offset = position + normal
    assert_eq!(layout.attributes[2].offset, 24);
    
    // color: offset = position + normal + tex_coord
    assert_eq!(layout.attributes[3].offset, 32);
    
    // alpha: offset = position + normal + tex_coord + color
    assert_eq!(layout.attributes[4].offset, 48);
    
    // bone_indices: offset = position + normal + tex_coord + color + alpha
    assert_eq!(layout.attributes[5].offset, 52);
    
    // tangent: offset = position + normal + tex_coord + color + alpha + bone_indices
    // This might require alignment padding depending on compiler
    assert_eq!(layout.attributes[6].offset, 56);
    
    // Check formats
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(layout.attributes[1].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(layout.attributes[2].format, wgpu::VertexFormat::Float32x2);
    assert_eq!(layout.attributes[3].format, wgpu::VertexFormat::Float32x4);
    assert_eq!(layout.attributes[4].format, wgpu::VertexFormat::Float32);
    // This should be Uint8x4 if correctly mapped
    assert_eq!(layout.attributes[5].format, wgpu::VertexFormat::Uint8x4);
    assert_eq!(layout.attributes[6].format, wgpu::VertexFormat::Float32x4);
    
    // Ensure step mode is Vertex
    assert_eq!(layout.step_mode, wgpu::VertexStepMode::Vertex);
    
    println!("Array stride alignment test passed!");
}
