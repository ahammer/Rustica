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

use rustica_ecs::World;
use rustica_ecs::Time;
use rustica_scheduler::Schedule;
use rustica_scheduler::System;
use rustica_scheduler::Stage;

use crate::Plugin;

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
    /// impl PluginMetadata for MyPlugin {}
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
    
    /// Takes a resource out of the app, returning it if it exists.
    /// 
    /// This is useful when you need to work with multiple resources at once.
    fn take_resource<R: 'static>(&mut self) -> Option<R> {
        let type_id = TypeId::of::<R>();
        self.resources.remove(&type_id)
            .and_then(|resource| resource.downcast::<R>().ok())
            .map(|boxed| *boxed)
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
        
        let mut last_update_time = std::time::Instant::now();
        
        // Check if we have a render window resource
        let has_window = self.get_resource::<bool>().copied().unwrap_or(false);
        
        if has_window {
            info!("Running with window");
            
            // Main loop for windowed applications
            while self.running {
                // Calculate delta time
                let current_time = std::time::Instant::now();
                let delta_time = current_time.duration_since(last_update_time);
                last_update_time = current_time;
                
                // Update time resource if present
                if let Some(mut time) = self.take_resource::<Time>() {
                    time.update(delta_time);
                    self.insert_resource(time);
                }
                
                // Process one frame
                self.update();
                
                // Simple sleep to avoid hogging CPU
                std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS cap
            }
        } else {
            info!("Running without window (headless mode)");
            // Do a single update for headless mode
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
        debug!("App update");
        
        // Check if we have the schedule and world resources
        let has_schedule = self.get_resource::<Schedule>().is_some();
        let has_world = self.get_resource::<World>().is_some();
        
        // Run all systems through the schedule
        if has_schedule && has_world {
            // Take the resources we need
            if let (Some(mut world), Some(mut schedule)) = (self.take_resource::<World>(), self.take_resource::<Schedule>()) {
                debug!("Running schedule on world");
                schedule.run(&mut world);
                
                // Put the resources back
                self.insert_resource(world);
                self.insert_resource(schedule);
            }
        } else if !has_schedule {
            debug!("No schedule resource found");
        } else {
            debug!("No world resource found, skipping schedule execution");
        }
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
    
    /// Adds a system to the application's schedule.
    ///
    /// This will register the system with the schedule, allowing it to be executed
    /// during the appropriate stage. If no schedule is present, it will create one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    /// use rustica_ecs::World;
    /// use rustica_scheduler::stage::Stage;
    ///
    /// fn my_system(world: &mut World) {
    ///     // System logic...
    /// }
    ///
    /// let mut app = App::new();
    /// app.add_system(my_system, "my_system", Stage::Update);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a result with the app instance if successful, or an error if the system
    /// could not be added.
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> &mut Self
    where
        S: System,
    {
        debug!("Adding system: {} in stage: {:?}", name, stage);
        
        // Check if we have a schedule resource
        let has_schedule = self.get_resource::<Schedule>().is_some();
        
        // If no schedule, create one
        if !has_schedule {
            debug!("No schedule found, creating a new one");
            self.insert_resource(Schedule::new());
        }
        
        // Now take the schedule and add the system
        if let Some(mut schedule) = self.take_resource::<Schedule>() {
            // Add the system to the schedule
            if let Err(e) = schedule.add_system(system, name, stage) {
                error!("Failed to add system '{}': {:?}", name, e);
            }
            
            // Put the schedule back
            self.insert_resource(schedule);
        } else {
            error!("Failed to get schedule resource even after creating it");
        }
        
        self
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// Implement InsertResource from rustica_ecs
impl rustica_ecs::plugin::InsertResource for App {
    fn insert_resource<R: 'static>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }
}

// Implement InsertResource from rustica_scheduler
impl rustica_scheduler::InsertResource for App {
    fn insert_resource<R: 'static>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
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
        
        impl rustica_common::PluginMetadata for TestPlugin {}
        
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
