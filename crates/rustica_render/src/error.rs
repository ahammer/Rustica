//! Error types for the rustica_render crate

use thiserror::Error;

/// A specialized Result type for rustica_render operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the rustica_render crate
#[derive(Error, Debug)]
pub enum Error {
    /// Renderer initialization failed
    #[error("Renderer initialization failed: {0}")]
    InitializationError(String),
    
    /// Resource loading failed
    #[error("Failed to load resource: {0}")]
    ResourceError(String),
    
    /// Shader compilation or linking failed
    #[error("Shader error: {0}")]
    ShaderError(String),
    
    /// Rendering operation failed
    #[error("Rendering error: {0}")]
    RenderError(String),
    
    /// Window-related error
    #[error("Window error: {0}")]
    WindowError(String),
    
    /// Generic error with message
    #[error("{0}")]
    Generic(String),
}
