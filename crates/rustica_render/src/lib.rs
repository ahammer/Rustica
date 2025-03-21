//! Rustica Rendering System
//!
//! This crate provides functionality for rendering in the Rustica game engine,
//! including the renderer, components, resources, and related types for the
//! starfield and other rendering tasks.

mod error;
mod renderer;
mod window;
mod input;

pub use error::{Error, Result};
pub use renderer::{
    Camera,
    Renderer, 
    RenderComponent, 
    RenderResource,
    StarComponent,
    StarPoint,
    Viewport,
};
pub use window::{
    WindowManager,
    WindowConfig,
    WindowResource,
};
pub use input::{
    InputState,
    InputResource,
    camera_input_system,
};

/// Re-exports of commonly used types
pub mod prelude {
    pub use crate::{
        Camera,
        Error, 
        Renderer, 
        RenderComponent, 
        RenderResource,
        Result, 
        StarComponent,
        StarPoint,
        Viewport,
        WindowManager,
        WindowConfig,
        WindowResource,
    };
}
