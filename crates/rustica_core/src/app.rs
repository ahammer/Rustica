//! # Application Lifecycle
//! 
//! This module defines the App struct, which is the central orchestration
//! point for the Rustica engine.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#AppLifecycle
//! - API_RULES: /docs/api_conventions.md#AppAPI
//! 
//! ## Critical Rules
//! 
//! 1. App should be minimal and delegate to subsystems via plugins
//! 2. Lifecycle methods should be clear and predictable
//! 3. Resource management should be explicit and safe

// === REGION: IMPORTS ===
use std::collections::HashMap;
use std::any::{Any, TypeId};
use log::{debug, error, info};

use crate::Plugin;
use crate::CoreError;

// === REGION: TYPE DEFINITIONS ===

/// A type-erased plugin stored in the App.
type BoxedPlugin = Box<dyn Any>;

/// A type-erased resource stored in the App.
type BoxedResource = Box<dyn Any>;

// === REGION: APP IMPLEMENTATION ===

/// The central application struct for the Rustica engine.
///
/// The App is responsible for:
/// - Managing the engine lifecycle
/// - Registering and running plugins
/// - Storing shared resources
/// - Orchestrating systems execution
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
///         // Configure the app
///     }
/// }
///
/// let mut app = App::new();
/// app.add_plugin(MyPlugin);
/// // app.run(); // Would start the main loop
/// ```
pub struct App {
    /// Registered plugins by name
    plugins: HashMap<String, BoxedPlugin>,
    
    /// Shared resources by type
    resources: HashMap<TypeId, BoxedResource>,
    
    /// Flag indicating if the app is running
    running: bool,
}

impl App {
    /// Creates a new, empty App instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        App {
            plugins: HashMap::new(),
            resources: HashMap::new(),
            running: false,
        }
    }
    
    /// Adds a plugin to the application.
    ///
    /// This will call the plugin's `build` method, allowing it to configure
    /// the application.
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
    ///         // Configure the app
    ///     }
    /// }
    ///
    /// let mut app = App::new();
    /// app.add_plugin(MyPlugin);
    /// ```
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        let name = plugin.name().to_string();
        debug!("Adding plugin: {}", name);
        
        // Check for duplicate plugins
        if self.plugins.contains_key(&name) {
            error!("Duplicate plugin: {}", name);
        // todo: fix this - return a Result instead of just logging and continuing
        }
        
        // Check plugin dependencies
        for dep in plugin.dependencies() {
            if !self.plugins.contains_key(dep) {
                error!("Missing plugin dependency: {} requires {}", name, dep);
                // todo: fix this - return a Result
            }
        }
        
        // Build the plugin
        plugin.build(self);
        
        // Store the plugin
        self.plugins.insert(name, Box::new(plugin));
        
        self
    }
    
    /// Inserts a resource into the application.
    ///
    /// Resources are shared data that can be accessed by systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// #[derive(Debug)]
    /// struct WindowConfig {
    ///     width: u32,
    ///     height: u32,
    /// }
    ///
    /// let mut app = App::new();
    /// app.insert_resource(WindowConfig {
    ///     width: 800,
    ///     height: 600,
    /// });
    /// ```
    pub fn insert_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
        self
    }
    
    /// Gets a reference to a resource, or None if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// #[derive(Debug)]
    /// struct WindowConfig {
    ///     width: u32,
    ///     height: u32,
    /// }
    ///
    /// let mut app = App::new();
    /// app.insert_resource(WindowConfig {
    ///     width: 800,
    ///     height: 600,
    /// });
    ///
    /// if let Some(config) = app.get_resource::<WindowConfig>() {
    ///     assert_eq!(config.width, 800);
    ///     assert_eq!(config.height, 600);
    /// }
    /// ```
    pub fn get_resource<R: 'static>(&self) -> Option<&R> {
        let type_id = TypeId::of::<R>();
        self.resources.get(&type_id)
            .and_then(|resource| resource.downcast_ref::<R>())
    }
    
    /// Gets a mutable reference to a resource, or None if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// #[derive(Debug)]
    /// struct WindowConfig {
    ///     width: u32,
    ///     height: u32,
    /// }
    ///
    /// let mut app = App::new();
    /// app.insert_resource(WindowConfig {
    ///     width: 800,
    ///     height: 600,
    /// });
    ///
    /// if let Some(config) = app.get_resource_mut::<WindowConfig>() {
    ///     config.width = 1024;
    /// }
    ///
    /// let config = app.get_resource::<WindowConfig>().unwrap();
    /// assert_eq!(config.width, 1024);
    /// ```
    pub fn get_resource_mut<R: 'static>(&mut self) -> Option<&mut R> {
        let type_id = TypeId::of::<R>();
        self.resources.get_mut(&type_id)
            .and_then(|resource| resource.downcast_mut::<R>())
    }
    
    /// Runs the application until stopped.
    ///
    /// This starts the main loop of the application, which will
    /// continue until `App::exit` is called.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// // Configure app...
    /// app.run();
    /// ```
    pub fn run(&mut self) {
        info!("Starting application");
        self.running = true;
        
        // Main loop
        while self.running {
            self.update();
        }
        
        info!("Application stopped");
    }
    
    /// Performs a single update of the application.
    ///
    /// This is useful for testing or for applications that want
    /// to control their own main loop.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// // Configure app...
    /// app.update(); // Run a single frame
    /// ```
    pub fn update(&mut self) {
        // todo: fix this - implement proper update cycle with:
        // 1. Run input systems
        // 2. Run update systems
        // 3. Run render systems
        // etc.
        
        // For now, this is a stub implementation
        debug!("App update");
        
        // If this were real, we'd delegate to scheduler for running systems
    }
    
    /// Signals the application to exit.
    ///
    /// This will cause the main loop in `run` to exit after the
    /// current frame completes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// // Configure app...
    /// app.exit(); // Signal app to stop
    /// ```
    pub fn exit(&mut self) {
        info!("Exiting application");
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(!app.running);
        assert!(app.plugins.is_empty());
        assert!(app.resources.is_empty());
    }
    
    #[test]
    fn test_app_add_plugin() {
        struct TestPlugin;
        impl Plugin for TestPlugin {
            fn build(&self, app: &mut App) {
                app.insert_resource(42u32);
            }
        }
        
        let mut app = App::new();
        app.add_plugin(TestPlugin);
        
        // Verify plugin was added
        assert_eq!(app.plugins.len(), 1);
        
        // Verify plugin's build method was called
        let resource = app.get_resource::<u32>();
        assert!(resource.is_some());
        assert_eq!(*resource.unwrap(), 42);
    }
    
    #[test]
    fn test_app_resources() {
        #[derive(Debug, PartialEq)]
        struct TestResource(u32);
        
        let mut app = App::new();
        app.insert_resource(TestResource(42));
        
        // Get resource
        let resource = app.get_resource::<TestResource>();
        assert!(resource.is_some());
        assert_eq!(resource.unwrap().0, 42);
        
        // Modify resource
        if let Some(resource) = app.get_resource_mut::<TestResource>() {
            resource.0 = 84;
        }
        
        // Verify modification
        let resource = app.get_resource::<TestResource>();
        assert_eq!(resource.unwrap().0, 84);
    }
    
    #[test]
    fn test_app_exit() {
        let mut app = App::new();
        app.running = true;
        app.exit();
        assert!(!app.running);
    }
}
