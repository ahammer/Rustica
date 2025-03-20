//! Rustica Rendering System
//!
//! This crate provides functionality for rendering in the Rustica game engine.

mod error;
mod plugin;
mod renderer;

pub use error::{Error, Result};
pub use plugin::RenderPlugin;
pub use renderer::{Renderer, RenderComponent, RenderResource};

/// Re-exports of commonly used types
pub mod prelude {
    pub use crate::{Error, Result, RenderPlugin, Renderer, RenderComponent, RenderResource};
}
