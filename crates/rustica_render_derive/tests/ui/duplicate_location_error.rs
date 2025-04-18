// Test for duplicate location detection
// This test verifies that the macro detects duplicate locations and fails with a proper error

use rustica_render_derive::ShaderProperties;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for duplicate locations")]
struct DuplicateLocationShader {
    #[vertex(location = 0)]
    position: [f32; 3],
    
    // This should cause a compile error since location 0 is already used
    #[vertex(location = 0)]
    texcoord: [f32; 2],
    
    #[vertex(location = 2)]
    normal: [f32; 3],
}

fn main() {
    // This code should never execute because the macro should fail at compile time
    let descriptor = DuplicateLocationShader::descriptor();
    println!("Test failed: duplicate locations were not detected!");
}
