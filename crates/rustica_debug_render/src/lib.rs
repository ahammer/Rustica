//! Debug rendering functionality for the Rustica game engine.
//!
//! This crate provides a simple wgpu-based rendering system for debugging
//! purposes, allowing visualization of game objects and state during development.

mod debug_renderer;
mod primitives;
mod star_renderer;
pub mod plugin;
pub mod error;
pub mod command;
pub mod systems;

// Re-export key components for ease of use
pub use debug_renderer::DebugRenderer;
pub use primitives::{Point, Line, Rectangle};
pub use error::{Error, Result};
pub use command::RenderCommandList;

/// Components for the debug renderer.
pub mod components {
    use cgmath::{Vector2 as Vec2, Vector3 as Vec3, Vector4 as Vec4};
    use rustica_ecs::component::Component;

    /// A component that makes an entity visually debuggable.
    #[derive(Debug, Clone)]
    pub struct DebugRenderComponent {
        /// The color to use for debug rendering.
        pub color: Vec4<f32>,
        /// The size to use for debug rendering.
        pub size: f32,
        /// Whether the component is visible.
        pub visible: bool,
    }

    impl Default for DebugRenderComponent {
        fn default() -> Self {
            Self {
                color: Vec4::new(1.0, 1.0, 1.0, 1.0), // White
                size: 5.0,
                visible: true,
            }
        }
    }

    /// A component that makes an entity renderable as a star.
    #[derive(Debug, Clone)]
    pub struct DebugStarComponent {
        /// The color of the star.
        pub color: Vec4<f32>,
        /// The size of the star.
        pub size: f32,
        /// The brightness of the star.
        pub brightness: f32,
        /// Whether the star is visible.
        pub visible: bool,
    }

    impl Default for DebugStarComponent {
        fn default() -> Self {
            Self {
                color: Vec4::new(1.0, 1.0, 1.0, 1.0), // White
                size: 2.0,
                brightness: 1.0,
                visible: true,
            }
        }
    }
}

/// Resources for the debug renderer.
pub mod resources {
    use std::sync::{Arc, Mutex};
    use crate::DebugRenderer;

    /// A resource that holds the debug renderer.
    #[derive(Clone)]
    pub struct DebugRendererResource {
        /// The debug renderer.
        renderer: Arc<Mutex<DebugRenderer>>,
    }

    impl DebugRendererResource {
        /// Create a new debug renderer resource.
        pub fn new(renderer: DebugRenderer) -> Self {
            Self {
                renderer: Arc::new(Mutex::new(renderer)),
            }
        }

        /// Get a reference to the debug renderer.
        pub fn renderer(&self) -> Arc<Mutex<DebugRenderer>> {
            self.renderer.clone()
        }
    }
}