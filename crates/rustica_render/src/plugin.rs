//! Plugin for integrating the rendering system with the Rustica engine

use rustica_core::plugin::Plugin;
use rustica_core::App;
use crate::renderer::Renderer;

/// Plugin that integrates the rendering system with the Rustica engine
pub struct RenderPlugin;

impl RenderPlugin {
    /// Create a new RenderPlugin
    pub fn new() -> Self {
        Self
    }
}

impl Default for RenderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // TODO Register itself with the app
    }
    
    fn name(&self) -> &str {
        "RenderPlugin"
    }
    
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["CorePlugin"]
    }
}
