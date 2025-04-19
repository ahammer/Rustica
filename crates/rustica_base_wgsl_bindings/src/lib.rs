//! Rust bindings for WGSL shaders used across the Rustica project.
//!
//! This crate uses a build script (`build.rs`) and the `wgsl_to_wgpu` crate
//! to automatically generate Rust code from `.wgsl` files located in the
//! `src/shaders/` directory.
//!
//! The generated code is organized into modules corresponding to the shader filenames
//! (e.g., `phong.wgsl` generates `src/shaders/phong.rs`). These modules are then
//! made available under the `shaders` module of this crate.

/// Contains the generated Rust modules for each WGSL shader.


/// A prelude module for conveniently importing commonly used items from the generated shaders.
pub mod prelude {
    // Example: pub use super::shaders::phong::{VertexInput, VertexOutput, CameraUniform};
    // Add exports here as needed when shaders are used.
}

// You can add other utility functions or types related to these shaders here if needed.
