// Test for storage binding support
// This test verifies that the shader properties macro correctly handles
// storage bindings (SSBOs), which are important for compute shaders

use rustica_render_derive::ShaderProperties;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for storage bindings")]
struct StorageBindingShader {
    // Vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    normal: [f32; 3],
    
    // Regular uniform buffer
    #[uniform(binding = 0)]
    view_proj: [[f32; 4]; 4],
    
    // Storage buffer binding (simulated as u32 since we're just testing binding indices)
    #[storage(binding = 1, read_only)]
    particles_in: u32,
    
    // Output storage buffer (read_write)
    #[storage(binding = 2)]
    particles_out: u32,
    
    // Auto-assigned storage binding
    #[storage]
    indirect_args: u32,
    
    // Texture binding after storage bindings
    #[uniform(binding = 4)]
    color_texture: u32,
}

fn main() {
    // Get the shader descriptor
    let descriptor = StorageBindingShader::descriptor();
    
    // Verify number of uniform and storage bindings
    assert_eq!(descriptor.uniforms.len(), 5, "Should have 5 total bindings: 1 uniform, 3 storage, 1 texture");
    
    // Check binding indices
    let mut found_view_proj = false;
    let mut found_particles_in = false;
    let mut found_particles_out = false;
    let mut found_indirect_args = false;
    let mut found_color_texture = false;
    
    for binding in &descriptor.uniforms {
        match binding.name.as_str() {
            "view_proj" => {
                found_view_proj = true;
                assert_eq!(binding.binding, 0, "view_proj should have binding 0");
            },
            "particles_in" => {
                found_particles_in = true;
                assert_eq!(binding.binding, 1, "particles_in should have binding 1");
            },
            "particles_out" => {
                found_particles_out = true;
                assert_eq!(binding.binding, 2, "particles_out should have binding 2");
            },
            "indirect_args" => {
                found_indirect_args = true;
                assert_eq!(binding.binding, 3, "indirect_args should have binding 3 (auto-assigned)");
            },
            "color_texture" => {
                found_color_texture = true;
                assert_eq!(binding.binding, 4, "color_texture should have binding 4");
            },
            _ => panic!("Unexpected binding name: {}", binding.name),
        }
    }
    
    // Verify all bindings were found
    assert!(found_view_proj, "view_proj binding not found");
    assert!(found_particles_in, "particles_in binding not found");
    assert!(found_particles_out, "particles_out binding not found");
    assert!(found_indirect_args, "indirect_args binding not found");
    assert!(found_color_texture, "color_texture binding not found");
    
    println!("Storage binding test passed!");
}
