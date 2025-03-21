//! Plugin for integrating debug rendering with the Rustica engine.
//!
//! This module provides the plugin that connects the debug renderer with the
//! rest of the engine, registering necessary systems and resources.

use rustica_core::Plugin;
use rustica_core::App;
use rustica_common::PluginMetadata;
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

use crate::systems::{debug_star_render_system, debug_render_process_system};
use rustica_scheduler::stage::Stage;

// Implement PluginMetadata trait to satisfy the Plugin bound
impl PluginMetadata for DebugRenderPlugin {
    fn name(&self) -> &str {
        "DebugRenderPlugin"
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["RenderPlugin"]
    }
}

impl Plugin for DebugRenderPlugin {
    fn build(&self, app: &mut App) {
        println!("DebugRenderPlugin registered");
        
        // 1. Register the RenderCommandList resource
        app.insert_resource(RenderCommandList::new());
        println!("- Registered RenderCommandList resource");
        
        // TEMPORARY: Commenting out system registration until they are properly wrapped
        // as SystemFn<F> to implement the System trait
        
        // // 2. Register the debug star render system in the LateUpdate stage
        // // This system converts entity components into render commands
        // app.add_system(
        //     debug_star_render_system,
        //     "debug_star_render_system",
        //     Stage::LateUpdate
        // );
        println!("- [DISABLED] debug_star_render_system in LateUpdate stage");
        
        // // 3. Register the debug render process system in the Render stage
        // // This system processes render commands and draws to the screen
        // app.add_system(
        //     debug_render_process_system,
        //     "debug_render_process_system",
        //     Stage::Render
        // );
        println!("- [DISABLED] debug_render_process_system in Render stage");
        
        // This establishes a clear pipeline:
        // Game State (entities) → Render Commands → GPU Calls
        // Without the renderer knowing about stars or the stars knowing about rendering
    }
}
