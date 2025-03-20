//! Rustica Rendering System
//!
//! This crate provides functionality for rendering in the Rustica game engine,
//! including the renderer, components, resources, and related types for the
//! starfield and other rendering tasks.

mod error;
mod plugin;
mod renderer;

pub use error::{Error, Result};
pub use plugin::RenderPlugin;
pub use renderer::{
    Camera,
    Renderer, 
    RenderComponent, 
    RenderResource,
    StarComponent,
    StarPoint,
    Viewport,
};

/// Re-exports of commonly used types
pub mod prelude {
    pub use crate::{
        Camera,
        Error, 
        Renderer, 
        RenderComponent, 
        RenderPlugin, 
        RenderResource,
        Result, 
        StarComponent,
        StarPoint,
        Viewport,
    };
}
