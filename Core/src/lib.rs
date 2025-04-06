//! Rustica Core - The central integration module for the Rustica engine
//! 
//! This module serves as the main entry point for the Rustica engine, re-exporting
//! all other engine crates and their preludes for easier access.

// Re-export all crates
pub use rustica_foundation as foundation;
pub use rustica_graphics as graphics;
pub use rustica_render as render;
pub use rustica_render_derive as render_derive;
pub use rustica_window as window;

#[cfg(feature = "ecs")]
pub use rustica_ecs as ecs;

/// The prelude module that re-exports common types from all Rustica crates.
/// Import this module to get access to all commonly used types.
pub mod prelude {
    pub use rustica_foundation::prelude::*;
    pub use rustica_graphics::prelude::*;
    pub use rustica_render::prelude::*;
    
    // Common window types
    pub use rustica_window::{
        Window,
        WindowConfig,
        EventHandler,
    };

    // Common render types
    pub use rustica_render::{
        Canvas,
        RenderWindow,
        CustomShader,
        ShaderDescriptor,
        VertexSemantic,
    };

    #[cfg(feature = "ecs")]
    pub use rustica_ecs::prelude::*;
}
