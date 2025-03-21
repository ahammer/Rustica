//! # Plugin System
//! 
//! This module defines common types and interfaces for the Rustica engine.

/// Resource trait - marker trait for types that can be stored as resources
pub trait Resource: 'static {}
impl<T: 'static> Resource for T {}

/// A common plugin interface trait
/// 
/// This trait is deliberately minimal to avoid circular dependencies.
/// Each crate can define its own specialized Plugin trait that extends
/// this one with build methods appropriate to that crate.
pub trait PluginMetadata: 'static {
    /// Returns the name of the plugin.
    ///
    /// This is used for debugging and logging purposes.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    
    /// Returns the plugin's dependencies.
    ///
    /// These are the names of other plugins that must be
    /// registered before this plugin.
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }
}
