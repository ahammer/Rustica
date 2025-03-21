//! Plugin for integrating debug rendering with the Rustica engine.
//!
//! This module provides the plugin that connects the debug renderer with the
//! rest of the engine, registering necessary systems and resources.

use rustica_core::plugin::Plugin;
use rustica_ecs::world::World;

use crate::components::{DebugRenderComponent, DebugStarComponent};
use crate::command::RenderCommandList;

/// The plugin for debug rendering integration.
pub struct DebugRenderPlugin {
    /// The background color to use when clearing the screen
    pub clear_color: [f32; 4],
}

impl DebugRenderPlugin {
    /// Create a new debug render plugin with default configuration
    pub fn new() -> Self {
        Self {
            clear_color: [0.0, 0.0, 0.05, 1.0], // Dark blue, good for starfields
        }
    }
}

impl Plugin for DebugRenderPlugin {
    fn build(&self, app: &mut rustica_core::app::App) {
        println!("DebugRenderPlugin registered");
        println!("- Would register DebugPointComponent");
        println!("- Would register RenderCommandList resource");
        println!("- Would register debug_star_render_system to generate commands from components");
        println!("- Would register debug_render_process_system to render commands");
        
        // In a full implementation with a working scheduler API, we would:
        // 1. Register DebugStarComponent and other debug components
        // 2. Add RenderCommandList as a resource
        // 3. Add a stage for generating render commands (using debug_star_render_system)
        // 4. Add a stage for processing render commands (using debug_render_process_system)
        
        // This establishes a clear pipeline:
        // Game State (entities) → Render Commands → GPU Calls
        // Without the renderer knowing about stars or the stars knowing about rendering
    }

    fn name(&self) -> &str {
        "DebugRenderPlugin"
    }
}
