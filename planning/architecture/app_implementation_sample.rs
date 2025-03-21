//! This is a sample implementation of the refactored App structure.
//! 
//! This file demonstrates what the App struct and its methods could look like
//! after refactoring to a more direct integration of core systems.
//! This is not meant to be compiled as-is, but to serve as a reference.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::time::Instant;

// Imports would come from the appropriate crates
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

use rustica_common::PluginMetadata;

/// A type-erased plugin stored in the App.
type BoxedPlugin = Box<dyn Any>;

/// A type-erased resource stored in the App.
type BoxedResource = Box<dyn Any>;

/// The Plugin trait defines the interface for extending the Rustica engine.
///
/// Plugins are used to modularize gameplay functionality and extend the engine
/// in a maintainable way. Each plugin can register resources, systems, and 
/// other components with the application.
pub trait Plugin: PluginMetadata {
    /// Configures the application with this plugin's functionality.
    ///
    /// This is called when the plugin is added to the application,
    /// allowing it to register resources, systems, and other components.
    fn build(&self, app: &mut App);
}

/// The central application struct for the Rustica engine.
///
/// The App is responsible for:
/// - Managing the engine lifecycle
/// - Registering and running plugins
/// - Storing shared resources
/// - Orchestrating systems execution
pub struct App {
    // Core engine components (direct members, not plugins)
    world: World,
    schedule: Schedule,
    event_system: EventSystem,
    
    // Optional renderer (feature gated)
    #[cfg(feature = "render")]
    renderer: Option<Renderer>,
    
    // Plugin registry (for actual game extensions)
    plugins: HashMap<String, BoxedPlugin>,
    
    // Resources (for data that doesn't fit into other categories)
    resources: HashMap<TypeId, BoxedResource>,
    
    // Runtime state
    running: bool,
    last_update: Instant,
}

impl App {
    /// Creates a new App instance with core systems initialized.
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
    /// Plugins extend the engine with game-specific functionality,
    /// like custom systems, components, and behaviors.
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        let name = plugin.name().to_string();
        
        // Check for duplicate plugins
        if self.plugins.contains_key(&name) {
            log::error!("Duplicate plugin: {}", name);
            // Return early instead of just logging
            return self;
        }
        
        // Check plugin dependencies
        for dep in plugin.dependencies() {
            if !self.plugins.contains_key(dep) {
                log::error!("Missing plugin dependency: {} requires {}", name, dep);
                // Return early with an error
                return self;
            }
        }
        
        // Build the plugin (allow it to extend the app)
        plugin.build(self);
        
        // Store the plugin
        self.plugins.insert(name, Box::new(plugin));
        
        self
    }
    
    //=== Direct access to core systems ===
    
    /// Gets a reference to the ECS world.
    pub fn world(&self) -> &World {
        &self.world
    }
    
    /// Gets a mutable reference to the ECS world.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    /// Gets a reference to the scheduler.
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    
    /// Gets a mutable reference to the scheduler.
    pub fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }
    
    /// Gets a reference to the event system.
    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }
    
    /// Gets a mutable reference to the event system.
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
    
    //=== Convenience methods for common operations ===
    
    /// Adds a system to the application's schedule.
    ///
    /// This registers the system with the schedule to be executed
    /// during the appropriate stage.
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> &mut Self
    where
        S: System,
    {
        if let Err(e) = self.schedule.add_system(system, name, stage) {
            log::error!("Failed to add system '{}': {:?}", name, e);
        }
        self
    }
    
    /// Registers a component type with the ECS world.
    pub fn register_component<T: Component>(&mut self) -> &mut Self {
        self.world.register_component::<T>();
        self
    }
    
    /// Registers an event type with the event system.
    pub fn register_event<E: Event>(&mut self) -> &mut Self {
        self.event_system.register_event::<E>();
        self
    }
    
    /// Inserts a resource into the application.
    pub fn insert_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
        self
    }
    
    /// Gets a reference to a resource.
    pub fn get_resource<R: 'static>(&self) -> Option<&R> {
        let type_id = TypeId::of::<R>();
        self.resources.get(&type_id)
            .and_then(|resource| resource.downcast_ref::<R>())
    }
    
    /// Gets a mutable reference to a resource.
    pub fn get_resource_mut<R: 'static>(&mut self) -> Option<&mut R> {
        let type_id = TypeId::of::<R>();
        self.resources.get_mut(&type_id)
            .and_then(|resource| resource.downcast_mut::<R>())
    }
    
    /// Takes a resource out of the app, returning it if it exists.
    fn take_resource<R: 'static>(&mut self) -> Option<R> {
        let type_id = TypeId::of::<R>();
        self.resources.remove(&type_id)
            .and_then(|resource| resource.downcast::<R>().ok())
            .map(|boxed| *boxed)
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
    
    //=== Lifecycle methods ===
    
    /// Performs a single update of the application.
    pub fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_update);
        self.last_update = current_time;
        
        // Update time resource if present
        if let Some(mut time) = self.take_resource::<Time>() {
            time.update(delta_time);
            self.insert_resource(time);
        }
        
        // Process events
        self.event_system.process_events(&mut self.world);
        
        // Run systems through the schedule
        self.schedule.run(&mut self.world);
        
        // Render if renderer is available
        #[cfg(feature = "render")]
        if let Some(renderer) = &mut self.renderer {
            renderer.render(&self.world);
        }
    }
    
    /// Runs the application until stopped.
    pub fn run(&mut self) {
        log::info!("Starting application");
        self.running = true;
        self.last_update = Instant::now();
        
        // Check if we have a render window
        #[cfg(feature = "render")]
        let has_window = self.renderer.is_some();
        
        #[cfg(not(feature = "render"))]
        let has_window = false;
        
        if has_window {
            log::info!("Running with window");
            
            // Main loop for windowed applications
            while self.running {
                self.update();
                
                // Simple sleep to avoid hogging CPU
                std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS cap
            }
        } else {
            log::info!("Running without window (headless mode)");
            // Do a single update for headless mode
            self.update();
        }
        
        log::info!("Application stopped");
    }
    
    /// Signals the application to exit.
    pub fn exit(&mut self) {
        log::info!("Exiting application");
        self.running = false;
    }
}

// Default implementation just calls new()
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// Example of backward compatibility traits
impl rustica_ecs::plugin::InsertResource for App {
    fn insert_resource<R: 'static>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }
}

impl rustica_scheduler::InsertResource for App {
    fn insert_resource<R: 'static>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }
}

// === Example usage ===

#[allow(dead_code)]
fn usage_example() {
    // Create app with core components already initialized
    let mut app = App::new()
        .with_max_entities(10000);
    
    #[cfg(feature = "render")]
    let app = app.with_window(WindowConfig {
        title: "My Game".to_string(),
        width: 800,
        height: 600,
    });
    
    // Direct access to core systems
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
    
    // Add systems directly to the appropriate stages
    app.add_system(movement_system, "movement", Stage::Update);
    
    // Register events directly
    app.register_event::<CollisionEvent>();
    
    // Add gameplay plugins (true extensions, not core functionality)
    app.add_plugin(GameplayPlugin);
    
    app.run();
}

// Example component
#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {}

// Example component
#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {}

// Example event
struct CollisionEvent {
    entity_a: u32,
    entity_b: u32,
}

impl Event for CollisionEvent {}

// Example system
fn movement_system(world: &mut World) {
    // System implementation
}

// Example gameplay plugin (actual extension, not core functionality)
struct GameplayPlugin;

impl PluginMetadata for GameplayPlugin {
    fn name(&self) -> &str {
        "GameplayPlugin"
    }
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        // Add gameplay-specific systems
        app.add_system(ai_system, "ai", Stage::Update);
        
        // Spawn initial game entities
        let _entity = app.world_mut().spawn()
            .add_component(Position { x: 0.0, y: 0.0 })
            .add_component(Velocity { x: 1.0, y: 0.0 })
            .build();
    }
}

// Example gameplay system
fn ai_system(_world: &mut World) {
    // AI implementation
}
