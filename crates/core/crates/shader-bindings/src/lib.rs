//! Foundational types and generation logic for Rustica shaders.
mod shader_bindings;

// Re-export the shader module for easier access
pub use shader_bindings::pbr;

/// Public API for using the PBR shader bindings.
///
/// This module provides a cleaner interface to the generated shader bindings
/// with documentation and examples.
pub mod pbr_shader {
    use crate::shader_bindings::pbr;
    
    /// Camera uniform containing view-projection matrix and camera position
    pub use pbr::CameraUniform;
    
    /// Model uniform containing model matrix and normal transform matrix
    pub use pbr::ModelUniform;
    
    /// Material uniform containing color and PBR properties
    pub use pbr::MaterialUniform;
    
    /// Initialization struct for creating material uniforms
    pub use pbr::MaterialUniformInit;
    
    /// Vertex input structure for the PBR shader
    pub use pbr::VertexInput;
    
    // Bind group types for GPU resource binding
    pub use pbr::{
        WgpuBindGroup0, WgpuBindGroup1, WgpuBindGroup2,
        WgpuBindGroup0Entries, WgpuBindGroup1Entries, WgpuBindGroup2Entries,
        WgpuBindGroup0EntriesParams, WgpuBindGroup1EntriesParams, WgpuBindGroup2EntriesParams,
    };
    
    // Constants for shader entry points
    pub use pbr::{ENTRY_VS_MAIN, ENTRY_FS_MAIN};
    
    // Helper functions for creating vertex and fragment shader entries
    pub use pbr::{vs_main_entry, fs_main_entry, vertex_state, fragment_state};
    
    /// Creates a shader module from the embedded WGSL shader source.
    pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
        pbr::create_shader_module_embed_source(device)
    }
    
    /// Creates a pipeline layout for the PBR shader.
    pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        pbr::create_pipeline_layout(device)
    }
}

#[cfg(test)]
mod tests {    
    use glam::{Mat3A, Mat4, Vec3A, Vec4};
    use crate::shader_bindings::pbr::*;

    #[test]
    fn check_generated_structs() {
        // Create test instances of generated structs
        let camera = CameraUniform {
            view_proj: Mat4::IDENTITY,
            position: Vec3A::ZERO,
        };

        let model = ModelUniform {
            model: Mat4::IDENTITY,
            normal_transform: Mat3A::IDENTITY,
        };

        let material_init = MaterialUniformInit {
            base_color_factor: Vec4::ONE,
            metallic_factor: 0.5,
            roughness_factor: 0.5,
        };
        
        let material: MaterialUniform = material_init.into();

        let vertex = VertexInput {
            position: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            uv: [0.0, 0.0],
        };
        
        // Verify struct sizes
        assert!(std::mem::size_of::<VertexInput>() > 0);
        assert!(std::mem::size_of::<CameraUniform>() > 0);
        assert!(std::mem::size_of::<ModelUniform>() > 0);
        assert!(std::mem::size_of::<MaterialUniform>() > 0);        
        
        // Verify bytemuck Pod trait implementation
        let _camera_bytes: &[u8] = bytemuck::bytes_of(&camera);
        let _model_bytes: &[u8] = bytemuck::bytes_of(&model);
        let _material_bytes: &[u8] = bytemuck::bytes_of(&material);
        let _vertex_bytes: &[u8] = bytemuck::bytes_of(&vertex);
        
        // Verify bind group layout descriptors
        assert!(WgpuBindGroup0::LAYOUT_DESCRIPTOR.entries.len() > 0);
        assert!(WgpuBindGroup1::LAYOUT_DESCRIPTOR.entries.len() > 0);
        assert!(WgpuBindGroup2::LAYOUT_DESCRIPTOR.entries.len() > 0);
        
        // Verify vertex buffer layout
        let vertex_layout = VertexInput::vertex_buffer_layout(wgpu::VertexStepMode::Vertex);
        assert_eq!(vertex_layout.attributes.len(), 3); // position, normal, uv
    }
}