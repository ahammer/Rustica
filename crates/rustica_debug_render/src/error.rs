//! Error types for the debug renderer.

use thiserror::Error;

/// Result type for debug renderer operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for debug renderer operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Window error.
    #[error("Window error: {0}")]
    WindowError(String),

    /// wgpu error.
    #[error("wgpu error: {0}")]
    WgpuError(String),

    /// Render error.
    #[error("Render error: {0}")]
    RenderError(String),

    /// Resource error.
    #[error("Resource error: {0}")]
    ResourceError(String),

    /// Initialization error.
    #[error("Initialization error: {0}")]
    InitializationError(String),

    /// Shader compilation error.
    #[error("Shader compilation error: {0}")]
    ShaderError(String),

    /// Pipeline creation error.
    #[error("Pipeline creation error: {0}")]
    PipelineError(String),

    /// Buffer error.
    #[error("Buffer error: {0}")]
    BufferError(String),

    /// Texture error.
    #[error("Texture error: {0}")]
    TextureError(String),

    /// Index out of bounds error.
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),

    /// Internal error.
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<wgpu::RequestDeviceError> for Error {
    fn from(err: wgpu::RequestDeviceError) -> Self {
        Self::WgpuError(format!("Device request failed: {}", err))
    }
}

impl From<wgpu::SurfaceError> for Error {
    fn from(err: wgpu::SurfaceError) -> Self {
        Self::WgpuError(format!("Surface error: {}", err))
    }
}

impl From<rustica_render::Error> for Error {
    fn from(err: rustica_render::Error) -> Self {
        match err {
            rustica_render::Error::WindowError(e) => Self::WindowError(e),
            rustica_render::Error::RenderError(e) => Self::RenderError(e),
            rustica_render::Error::ResourceError(e) => Self::ResourceError(e),
            _ => Self::InternalError(format!("Render error: {}", err))
        }
    }
}

impl From<wgpu::CreateSurfaceError> for Error {
    fn from(err: wgpu::CreateSurfaceError) -> Self {
        Self::WgpuError(format!("Surface creation error: {}", err))
    }
}

/// Create an index out of bounds error
pub fn index_out_of_bounds(message: impl Into<String>) -> Error {
    Error::IndexOutOfBounds(message.into())
}
