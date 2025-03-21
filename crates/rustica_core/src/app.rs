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
use std::time::Instant;
use log::{debug, error, info};

use rustica_ecs::World;
use rustica_ecs::Time;
use rustica_ecs::Component;
use rustica_scheduler::Schedule;
use rustica_scheduler::System;
use rustica_scheduler::Stage;
use rustica_event::EventSystem;
use rustica_event::Event;

#[cfg(feature = "render")]
use rustica_render::Renderer;
#[cfg(feature = "render")]
use rustica_render::WindowConfig;

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
/// - Orchestrating core systems (World, Schedule, EventSystem)
/// - Registering and running plugins
/// - Storing shared resources
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
    /// Core ECS world
    world: World,
    
    /// Core scheduling system
    schedule: Schedule,
    
    /// Core event system
    event_system: EventSystem,
    
    /// Optional renderer (feature gated)
    #[cfg(feature = "render")]
    renderer: Option<Renderer>,
    
    /// Registered plugins by name
    plugins: HashMap<String, BoxedPlugin>,
    
    /// Shared resources by type
    resources: HashMap<TypeId, BoxedResource>,
    
    /// Flag indicating if the app is running
    running: bool,
    
    /// Last update time for delta time calculation
    last_update: Instant,
}

impl App {
    /// Creates a new App instance with core systems initialized.
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
            // Initialize core components directly
            world: World::new(),
            schedule: Schedule::new(),
            event_system: EventSystem::new(),
            
            #[cfg(feature = "render")]
            renderer: None,
            
            plugins: HashMap::new(),
            resources: HashMap::new(),
            
            running: false,
            last_update: Instant::now(),
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
    
    //=== Direct access to core systems ===
    
    /// Gets a reference to the ECS world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let app = App::new();
    /// let world = app.world();
    /// ```
    pub fn world(&self) -> &World {
        &self.world
    }
    
    /// Gets a mutable reference to the ECS world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// let world = app.world_mut();
    /// // Modify the world...
    /// ```
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    /// Gets a reference to the scheduler.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let app = App::new();
    /// let schedule = app.schedule();
    /// ```
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    
    /// Gets a mutable reference to the scheduler.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// let schedule = app.schedule_mut();
    /// // Modify the schedule...
    /// ```
    pub fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }
    
    /// Gets a reference to the event system.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let app = App::new();
    /// let event_system = app.event_system();
    /// ```
    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }
    
    /// Gets a mutable reference to the event system.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    ///
    /// let mut app = App::new();
    /// let event_system = app.event_system_mut();
    /// // Interact with the event system...
    /// ```
    pub fn event_system_mut(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
    
    #[cfg(feature = "render")]
    /// Gets a reference to the renderer, if one exists.
    pub fn renderer(&self) -> Option<&Renderer> {
        self.renderer.as_ref()
    }
    
    #[cfg(feature = "render")]
    /// Gets a mutable reference to the renderer, if one exists.
    pub fn renderer_mut(&mut self) -> Option<&mut Renderer> {
        self.renderer.as_mut()
    }
    
    //=== Builder pattern methods for configuration ===
    
    #[cfg(feature = "render")]
    /// Configures the app with a window and renderer.
    pub fn with_window(mut self, window_config: WindowConfig) -> Self {
        self.renderer = Some(Renderer::new(window_config));
        self
    }
    
    /// Configures the world with a specific entity capacity.
    pub fn with_max_entities(mut self, max_entities: usize) -> Self {
        self.world = World::with_capacity(max_entities);
        self
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
        self.last_update = Instant::now();
        
        // Check if we have a window
        #[cfg(feature = "render")]
        let has_window = self.renderer.is_some();
        
        #[cfg(not(feature = "render"))]
        let has_window = false;
        
        if has_window {
            info!("Running with window");
            
            // Main loop for windowed applications
            while self.running {
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
        
        // Update time resource if present
        if let Some(mut time) = self.take_resource::<Time>() {
            time.update(self.last_update.elapsed());
            self.last_update = Instant::now();
            self.insert_resource(time);
        }
        
        // Process events
        self.event_system.process_events(&mut self.world);
        
        // Run systems through the schedule
        self.schedule.run(&mut self.world);
        
        // Render if enabled
        #[cfg(feature = "render")]
        if let Some(renderer) = &mut self.renderer {
            renderer.render(&self.world);
        }
    }
    
    /// Registers a component type with the ECS world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    /// use rustica_ecs::Component;
    ///
    /// #[derive(Debug)]
    /// struct Position {
    ///     x: f32,
    ///     y: f32,
    /// }
    ///
    /// impl Component for Position {}
    ///
    /// let mut app = App::new();
    /// app.register_component::<Position>();
    /// ```
    pub fn register_component<T: Component>(&mut self) -> &mut Self {
        self.world.register_component::<T>();
        self
    }
    
    /// Registers an event type with the event system.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    /// use rustica_event::Event;
    ///
    /// struct CollisionEvent {
    ///     entity_a: u32,
    ///     entity_b: u32,
    /// }
    ///
    /// impl Event for CollisionEvent {}
    ///
    /// let mut app = App::new();
    /// app.register_event::<CollisionEvent>();
    /// ```
    pub fn register_event<E: Event>(&mut self) -> &mut Self {
        self.event_system.register_event::<E>();
        self
    }
    
    /// Sends an event to the event system.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_core::App;
    /// use rustica_event::Event;
    ///
    /// struct CollisionEvent {
    ///     entity_a: u32,
    ///     entity_b: u32,
    /// }
    ///
    /// impl Event for CollisionEvent {}
    ///
    /// let mut app = App::new();
    /// app.register_event::<CollisionEvent>();
    /// app.send_event(CollisionEvent { entity_a: 1, entity_b: 2 });
    /// ```
    pub fn send_event<E: Event>(&mut self, event: E) -> &mut Self {
        self.event_system.send_event(event);
        self
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
    /// during the appropriate stage.
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
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> &mut Self
    where
        S: System,
    {
        debug!("Adding system: {} in stage: {:?}", name, stage);
        
        // Add the system directly to the schedule field
        if let Err(e) = self.schedule.add_system(system, name, stage) {
            error!("Failed to add system '{}': {:?}", name, e);
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
    
    #[test]
    fn test_direct_access_methods() {
        let mut app = App::new();
        
        // Test world access
        assert!(app.world().entities().is_empty());
        
        // Test schedule access
        let schedule = app.schedule();
        assert_eq!(schedule.system_count(), 0);
        
        // Test event system access
        let event_system = app.event_system();
        // Assert properties of event system
        
        // Test builder pattern methods
        let app = App::new().with_max_entities(1000);
        assert_eq!(app.world().capacity(), 1000);
        
        #[cfg(feature = "render")]
        {
            let app = App::new().with_window(WindowConfig {
                title: "Test Window".to_string(),
                width: 800,
                height: 600,
            });
            assert!(app.renderer().is_some());
        }
    }
    
    #[test]
    fn test_register_component() {
        #[derive(Debug)]
        struct TestComponent;
        
        impl Component for TestComponent {}
        
        let mut app = App::new();
        app.register_component::<TestComponent>();
        
        // Verify component was registered
        assert!(app.world().is_component_registered::<TestComponent>());
    }
    
    #[test]
    fn test_add_system() {
        fn test_system(_world: &mut World) {
            // Test system logic
        }
        
        let mut app = App::new();
        app.add_system(test_system, "test_system", Stage::Update);
        
        // Verify system was added
        assert_eq!(app.schedule().system_count(), 1);
    }
}
