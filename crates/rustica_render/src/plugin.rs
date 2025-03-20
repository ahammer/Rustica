//! Plugin for integrating the rendering system with the Rustica engine
//!
//! This plugin initializes the renderer and registers it with the application,
//! allowing other systems to perform rendering operations.

use rustica_core::plugin::Plugin;
use rustica_core::App;
use crate::renderer::{Camera, Renderer, Viewport};
use crate::Result;
use log::{info, error};

/// Configuration for the render plugin
#[derive(Debug, Clone)]
pub struct RenderPluginConfig {
    /// Initial viewport settings
    pub viewport: Viewport,
    /// Initial camera settings
    pub camera: Camera,
    /// Whether to enable depth testing
    pub depth_testing: bool,
}

impl Default for RenderPluginConfig {
    fn default() -> Self {
        Self {
            viewport: Viewport::default(),
            camera: Camera::default(),
            depth_testing: true,
        }
    }
}

/// Plugin that integrates the rendering system with the Rustica engine
pub struct RenderPlugin {
    /// Configuration for the plugin
    config: RenderPluginConfig,
}

impl RenderPlugin {
    /// Create a new RenderPlugin with default configuration
    pub fn new() -> Self {
        Self {
            config: RenderPluginConfig::default(),
        }
    }
    
    /// Create a new RenderPlugin with custom configuration
    pub fn with_config(config: RenderPluginConfig) -> Self {
        Self {
            config,
        }
    }
    
    /// Initialize the renderer
    fn init_renderer(&self) -> Result<Renderer> {
        let mut renderer = Renderer::new();
        
        // Apply configuration
        renderer.set_viewport(self.config.viewport);
        renderer.set_camera(self.config.camera.clone());
        renderer.set_depth_testing(self.config.depth_testing);
        
        // Initialize the renderer
        match renderer.initialize() {
            Ok(_) => {
                info!("Renderer initialized successfully");
                Ok(renderer)
            },
            Err(e) => {
                error!("Failed to initialize renderer: {}", e);
                Err(e)
            }
        }
    }
}

impl Default for RenderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Initialize and add the renderer as a resource
        match self.init_renderer() {
            Ok(renderer) => {
                app.insert_resource(renderer);
                info!("RenderPlugin: Renderer added as resource");
                
                // Register systems for rendering
                // This would typically be done using the scheduler
                // For example:
                // app.add_system(render_stars_system);
            },
            Err(e) => {
                error!("RenderPlugin: Failed to initialize renderer: {}", e);
                // In a real implementation, we might want to handle this error more gracefully
            }
        }
    }
    
    fn name(&self) -> &str {
        "RenderPlugin"
    }
    
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["CorePlugin"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_render_plugin_default() {
        let plugin = RenderPlugin::default();
        assert_eq!(plugin.name(), "RenderPlugin");
        assert_eq!(plugin.dependencies(), vec!["CorePlugin"]);
    }
    
    #[test]
    fn test_render_plugin_with_config() {
        let config = RenderPluginConfig {
            viewport: Viewport {
                width: 1024,
                height: 768,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let plugin = RenderPlugin::with_config(config);
        assert_eq!(plugin.config.viewport.width, 1024);
        assert_eq!(plugin.config.viewport.height, 768);
    }
}
