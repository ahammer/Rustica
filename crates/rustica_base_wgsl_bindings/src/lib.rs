//! Rust bindings for WGSL shaders used across the Rustica project.
//! Generated by `build.rs` using `wgsl_to_wgpu`.

// This module directly includes the generated module declarations (pub mod phong; pub mod other_shader;)
// Access items directly via: crate::shaders::phong::VertexInput
pub mod shaders {
    // This include macro inserts the content of the file generated by build.rs
    // which contains `pub mod <shader_name>;` declarations.
    pub mod phong {
        include!("phong.rs");
    }
}

// You can add other utility functions or types related to these shaders here if needed.
