// Diagnostic test to determine what's happening with vertex formats
// This will print actual values rather than asserting

use rustica_render_derive::ShaderProperties;
use rustica_render::Vertex;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for vertex format diagnostics")]
struct DiagnosticFormatShader {
    #[vertex(location = 0)]
    position_2d: [f32; 2],
    
    #[vertex(location = 1)]
    normal: [f32; 3],
    
    #[vertex(location = 2)]
    color: [f32; 4],
}

fn main() {
    // Get the generated vertex layout
    let layout = DiagnosticFormatShaderVertex::layout();
    
    // Print all vertex attribute details for diagnosis
    println!("\n=== VERTEX FORMAT DIAGNOSTIC RESULTS ===");
    println!("Struct size: {} bytes", std::mem::size_of::<DiagnosticFormatShaderVertex>());
    
    for (i, attr) in layout.attributes.iter().enumerate() {
        println!("\nAttribute[{}]:", i);
        println!("  Format:          {:?}", attr.format);
        println!("  Offset:          {} bytes", attr.offset);
        println!("  Shader location: {}", attr.shader_location);
    }
    
    // Print what we expected vs what we got
    println!("\n=== EXPECTED VS ACTUAL ===");
    println!("Attribute[0] format - Expected: Float32x2, Actual: {:?}", layout.attributes[0].format);
    println!("Attribute[1] format - Expected: Float32x3, Actual: {:?}", layout.attributes[1].format);
    println!("Attribute[2] format - Expected: Float32x4, Actual: {:?}", layout.attributes[2].format);
    
    println!("Attribute[0] offset - Expected: 0 bytes,  Actual: {} bytes", layout.attributes[0].offset);
    println!("Attribute[1] offset - Expected: 8 bytes,  Actual: {} bytes", layout.attributes[1].offset);
    println!("Attribute[2] offset - Expected: 20 bytes, Actual: {} bytes", layout.attributes[2].offset);
    
    println!("\nThis diagnostic explains why vertex positions in GeometryWithInstances may be corrupted!");
    println!("=== END DIAGNOSTIC ===\n");
}