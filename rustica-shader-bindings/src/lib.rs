//! Foundational types and generation logic for Rustica shaders.

// Include the generated bindings module
#[allow(clippy::all)] // More specific allow for clippy warnings in generated code
mod pbr_bindings {
    include!(concat!(env!("OUT_DIR"), "/pbr_bindings.rs"));
}

// Re-export the shader module for easier access
pub use pbr_bindings::pbr;

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat3A, Mat4, Vec3A, Vec4};
    // Import the nested types
    use pbr_bindings::pbr::*;

    // Basic test to ensure generated structs exist and have expected fields/types
    #[test]
    fn check_generated_structs() {
        // Use the correct field types from the generated code
        let camera = CameraUniform {
            view_proj: Mat4::IDENTITY,
            position: Vec3A::ZERO,
        };

        let model = ModelUniform {
            model: Mat4::IDENTITY,
            normal_transform: Mat3A::IDENTITY,
        };

        // Use MaterialUniformInit for easier initialization
        let material_init = MaterialUniformInit {
            base_color_factor: Vec4::ONE,
            metallic_factor: 0.5,
            roughness_factor: 0.5,
        };
        
        // Convert to the actual struct which handles padding
        let material: MaterialUniform = material_init.into();

        // Create a vertex input with correct types
        let vertex = VertexInput {
            position: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            uv: [0.0, 0.0],
        };        // Check size and alignment (basic sanity check)
        assert!(std::mem::size_of::<VertexInput>() > 0);
        assert!(std::mem::size_of::<CameraUniform>() > 0);
        assert!(std::mem::size_of::<ModelUniform>() > 0);
        assert!(std::mem::size_of::<MaterialUniform>() > 0);        
        
        // Check if Bytemuck derives worked by verifying that we can cast to byte slices
        // This requires the Pod trait to be implemented
        let _camera_bytes: &[u8] = bytemuck::bytes_of(&camera);
        let _model_bytes: &[u8] = bytemuck::bytes_of(&model);
        let _material_bytes: &[u8] = bytemuck::bytes_of(&material);
        let _vertex_bytes: &[u8] = bytemuck::bytes_of(&vertex);
        
        // Check the layout entries are accessible
        assert!(WgpuBindGroup0::LAYOUT_DESCRIPTOR.entries.len() > 0);
        assert!(WgpuBindGroup1::LAYOUT_DESCRIPTOR.entries.len() > 0);
        assert!(WgpuBindGroup2::LAYOUT_DESCRIPTOR.entries.len() > 0);
        
        // Check vertex buffer layouts
        let vertex_layout = VertexInput::vertex_buffer_layout(wgpu::VertexStepMode::Vertex);
        assert_eq!(vertex_layout.attributes.len(), 3); // position, normal, uv
    }
}
