//! Plugin for integrating the event system with the Rustica engine

use rustica_core::Plugin;
use rustica_core::App;
use rustica_common::PluginMetadata;
use crate::event::Events;

/// Plugin that integrates the event system with the Rustica engine
pub struct EventPlugin;

impl EventPlugin {
    /// Create a new EventPlugin
    pub fn new() -> Self {
        Self
    }
}

impl Default for EventPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// Implement PluginMetadata trait to satisfy the Plugin bound
impl PluginMetadata for EventPlugin {
    fn name(&self) -> &str {
        "EventPlugin"
    }

    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }
}

impl Plugin for EventPlugin {
    fn build(&self, _app: &mut App) {
        // Register core event types and systems
        // This is a placeholder implementation
        // app.add_resource(Events::<()>::default());
        
        // For now we're just stubbing this since we don't have access to app.add_resource yet
    }
}
