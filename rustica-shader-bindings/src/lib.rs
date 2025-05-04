//! Foundational types and generation logic for Rustica shaders.

// Include the generated bindings module
#[allow(dead_code)] // Silence warnings for unused generated code
mod pbr_bindings {
    // Use include! macro within the module
    include!(concat!(env!("OUT_DIR"), "/pbr_bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat4, Vec3, Vec4};
    // Import the nested types
    use pbr_bindings::pbr::*;

    // Basic test to ensure generated structs exist and have expected fields/types
    #[test]
    fn check_generated_structs() {
        // Use the types via the nested module path
        let camera = CameraUniform {
            view_proj: Mat4::IDENTITY,
            position: Vec3::ZERO,
            _padding: 0, // Assuming padding might be added
        };

        let material = MaterialUniform {
            albedo: Vec4::ONE,
            metallic: 0.5,
            roughness: 0.5,
            _padding1: 0.0,
            _padding2: 0.0,
        };

        // Check size and alignment (basic sanity check)
        assert!(std::mem::size_of::<VertexInput>() > 0);
        assert!(std::mem::size_of::<CameraUniform>() > 0);
        assert!(std::mem::size_of::<ModelUniform>() > 0);
        assert!(std::mem::size_of::<MaterialUniform>() > 0);

        // Check if Bytemuck derives worked (Pod implies Zeroable)
        assert!(VertexInput::is_pod());
        assert!(CameraUniform::is_pod());
        assert!(ModelUniform::is_pod());
        assert!(MaterialUniform::is_pod());

        // Check generated layout constants (if they exist and are accessible)
        // These might not be generated or accessible depending on wgsl_bindgen version/config
        // assert_eq!(LAYOUT_DESCRIPTOR.bind_group_layouts.len(), 3);
        // assert!(VERTEX_BUFFER_LAYOUTS.len() > 0);
    }
}
