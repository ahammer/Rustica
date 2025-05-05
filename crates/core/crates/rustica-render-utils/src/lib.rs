//! # Rustica Render Utils
//!
//! Simple utilities for working with shader bindings and WGPU resources.
//! This crate aims to reduce boilerplate when creating common rendering resources
//! while still giving direct access to the underlying WGPU objects.
//!
//! ## Example
//!
//! ```
//! use rustica_render_utils::{create_camera_resources, update_camera};
//! use glam::{Mat4, Vec3A};
//! # fn example(device: &wgpu::Device, queue: &wgpu::Queue) {
//!   // Create camera resources
//!   let (camera_buffer, camera_bind_group) = create_camera_resources(device);
//!   
//!   // Update camera with new view
//!   let view_proj = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
//!   update_camera(queue, &camera_buffer, view_proj, Vec3A::new(0.0, 0.0, 1.0));
//! # }
//! ```

mod resources;
mod geometry;
mod projection;
mod pipeline;

pub use resources::{
    create_camera_resources, update_camera,
    create_model_resources, update_model_transform,
    create_material_resources
};
pub use geometry::create_vertex_buffer;
pub use projection::create_orthographic_projection;
pub use pipeline::create_pipeline;

// Re-export types from shader bindings for convenience
pub use rustica_shader_bindings::pbr_shader::{
    MaterialUniform, MaterialUniformInit, VertexInput
};
