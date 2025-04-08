// Test for texture binding support
// This test verifies that texture samplers can be properly defined
// in shader properties and are assigned correct bindings

use rustica_render_derive::ShaderProperties;
use rustica_foundation::Vertex;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for texture binding support")]
struct TextureBindingShader {
    // Vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    tex_coords: [f32; 2],
    
    // Regular uniform bindings
    #[uniform(binding = 0)]
    view_projection: [[f32; 4]; 4],
    
    // Texture sampler binding - should be treated as a regular uniform
    // with appropriate binding number
    #[uniform(binding = 1)]
    texture: u32, // In WGSL, this would be a texture_2d<f32>
    
    // Texture sampler binding - automatically assigned binding
    #[uniform]
    sampler: u32, // In WGSL, this would be a sampler
    
    // Another uniform after textures
    #[uniform]
    color_tint: [f32; 4],
}

fn main() {
    // Get the shader descriptor
    let descriptor = TextureBindingShader::descriptor();
    
    // Verify number of uniforms
    assert_eq!(descriptor.uniforms.len(), 4, "Should have 4 uniforms including textures and samplers");
    
    // Check uniform bindings
    let mut found_view_proj = false;
    let mut found_texture = false;
    let mut found_sampler = false;
    let mut found_color_tint = false;
    
    for uniform in &descriptor.uniforms {
        match uniform.name.as_str() {
            "view_projection" => {
                found_view_proj = true;
                assert_eq!(uniform.binding, 0, "view_projection should have binding 0");
            },
            "texture" => {
                found_texture = true;
                assert_eq!(uniform.binding, 1, "texture should have binding 1");
            },
            "sampler" => {
                found_sampler = true;
                assert_eq!(uniform.binding, 2, "sampler should have binding 2");
            },
            "color_tint" => {
                found_color_tint = true;
                assert_eq!(uniform.binding, 3, "color_tint should have binding 3");
            },
            _ => panic!("Unexpected uniform name: {}", uniform.name),
        }
    }
    
    // Verify all uniforms were found
    assert!(found_view_proj, "view_projection uniform not found");
    assert!(found_texture, "texture uniform not found");
    assert!(found_sampler, "sampler uniform not found");
    assert!(found_color_tint, "color_tint uniform not found");
    
    // Verify automatic binding assignment worked as expected
    // (sampler got binding 2, and color_tint got binding 3)
    
    println!("Texture binding support test passed successfully!");
}
