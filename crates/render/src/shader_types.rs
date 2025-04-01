// Shader type definitions for the rendering system

/// Shader types available in the engine
/// 
/// # Deprecated
/// This enum is deprecated and will be removed in a future version.
/// Use the modern shader API with `draw_with_shader` instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    since = "0.2.0",
    note = "This is being replaced by the custom shader API. Use `draw_with_shader` instead."
)]
pub enum ShaderType {
    /// Basic shader with vertex colors
    DebugColor,
}
