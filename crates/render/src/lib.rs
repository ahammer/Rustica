// Rendering module for Rustica engine

// Re-export derive macros when the "derive" feature is enabled
#[cfg(feature = "derive")]
pub use rustica_render_derive::*;

// Re-export Foundation geometry types
pub use rustica_foundation::geometry::{Vertex, VertexAttributeProvider, VertexAttribute, Triangle};

// Re-export public API
pub use canvas::Canvas;
pub use render_window::RenderWindow;
pub use custom_shader::{CustomShader, ShaderDescriptor, UniformParameter};
pub use standard_mesh::StandardMeshAdapter;

// Internal modules
mod shader_types; // Empty file kept for backward compatibility
mod draw_commands;
mod canvas;
mod render_context;
mod render_window;
mod shaders; // Simplified module, no built-in shaders
mod custom_shader;
mod standard_mesh;
pub mod prelude;

#[cfg(test)]
mod tests {
    // Re-export tests module
}
