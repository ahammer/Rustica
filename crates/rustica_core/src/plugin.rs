//! # Plugin System
//! 
//! This module defines the Plugin trait, which is the primary extension
//! mechanism for the Rustica engine.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#PluginSystem
//! - API_RULES: /docs/api_conventions.md#PluginAPI
//! 
//! ## Critical Rules
//! 
//! 1. Plugins must be composable and have clear responsibilities
//! 2. Plugins should declare their dependencies explicitly
//! 3. Plugin initialization must be idempotent when possible

// === REGION: IMPORTS ===
use std::any::type_name;
use crate::App;

// === REGION: PLUGIN TRAIT ===

/// The primary extension point for the Rustica engine.
///
/// Plugins are used to add functionality to the engine in a modular way.
/// Each plugin can register components, systems, resources, or other 
/// engine features during the build phase.
///
/// # Examples
///
/// ```
/// use rustica_core::prelude::*;
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         // Add functionality to the app
///     }
///
///     fn dependencies(&self) -> Vec<&'static str> {
///         // Declare dependencies on other plugins
///         vec!["CorePlugin"]
///     }
/// }
/// ```
pub trait Plugin: 'static {
    /// Called when the plugin is added to the application.
    ///
    /// This is where the plugin should register its components,
    /// systems, resources, etc.
    fn build(&self, app: &mut App);
    
    /// Returns the name of this plugin.
    ///
    /// By default, returns the type name, but can be overridden
    /// to provide a custom name.
    fn name(&self) -> &str {
        type_name::<Self>()
    }
    
    /// Returns a list of plugins that this plugin depends on.
    ///
    /// The application will ensure these plugins are registered
    /// before this plugin is built.
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;
    
    #[test]
    fn test_plugin_name() {
        struct TestPlugin;
        impl Plugin for TestPlugin {
            fn build(&self, _app: &mut App) {}
        }
        
        let plugin = TestPlugin;
        assert_eq!(plugin.name(), "rustica_core::plugin::tests::test_plugin_name::TestPlugin");
    }
    
    #[test]
    fn test_plugin_dependencies() {
        struct TestPlugin;
        impl Plugin for TestPlugin {
            fn build(&self, _app: &mut App) {}
            
            fn dependencies(&self) -> Vec<&'static str> {
                vec!["CorePlugin", "RenderPlugin"]
            }
        }
        
        let plugin = TestPlugin;
        let deps = plugin.dependencies();
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0], "CorePlugin");
        assert_eq!(deps[1], "RenderPlugin");
    }
}
