// Rendering module for Rustica engine

// Re-export Foundation geometry types
pub use rustica_foundation::geometry::{Vertex, VertexAttributeProvider, VertexAttribute, Triangle};

// Semantic types for vertex attributes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VertexSemantic {
    Position,
    Normal,
    Color,
    TexCoord,
    Tangent,
    Bitangent,
    // Add other semantics as needed
}

// Re-export public API
pub use canvas::Canvas;
pub use render_window::RenderWindow;
pub use custom_shader::{CustomShader, ShaderDescriptor, UniformParameter};

// Internal modules
mod draw_commands;
mod canvas;
mod render_context;
mod render_window;
mod shaders; // Simplified module, no built-in shaders
mod custom_shader;
pub mod prelude;

#[cfg(test)]
mod tests {
    // Re-export tests module
}
