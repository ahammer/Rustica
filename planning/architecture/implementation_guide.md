# Rustica Architecture Implementation Guide

This document provides detailed implementation steps for refactoring the Rustica engine architecture according to the vision laid out in the refactoring plan. It outlines specific code changes required for each component to move from a plugin-centric architecture to one with clear boundaries between core functionality and plugins.

## Core Implementation: rustica_core

### App Structure Changes

```rust
// Current App struct (simplified)
pub struct App {
    plugins: HashMap<String, BoxedPlugin>,
    resources: HashMap<TypeId, BoxedResource>,
    running: bool,
}

// Proposed App struct
pub struct App {
    // Core engine components (direct members, not plugins)
    world: World,
    schedule: Schedule,
    event_system: EventSystem,
    
    // Optional renderer (feature gated)
    #[cfg(feature = "render")]
    renderer: Option<Renderer>,
    
    // Plugin registry (for actual extensions)
    plugins: HashMap<String, BoxedPlugin>,
    
    // Resources (for data that doesn't fit into other categories)
    resources: HashMap<TypeId, BoxedResource>,
    
    // Runtime state
    running: bool,
}
```

### App Implementation Changes

```rust
impl App {
    // Updated constructor
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
        }
    }
    
    // Direct access methods for core components
    pub fn world(&self) -> &World {
        &self.world
    }
    
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    
    pub fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }
    
    // Simplified system registration (direct, not through plugins)
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> &mut Self
    where
        S: System,
    {
        self.schedule.add_system(system, name, stage).expect("Failed to add system");
        self
    }
    
    // Simplified update method
    pub fn update(&mut self) {
        // Update time resource if present
        if let Some(mut time) = self.take_resource::<Time>() {
            time.update(std::time::Instant::now() - self.last_update);
            self.insert_resource(time);
        }
        
        // Process events
        self.event_system.process_events(&mut self.world);
        
        // Run systems
        self.schedule.run(&mut self.world);
        
        // Render if enabled
        #[cfg(feature = "render")]
        if let Some(renderer) = &mut self.renderer {
            renderer.render(&self.world);
        }
    }
    
    // Plugin system remains but focused on extensions
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        // Similar to current implementation, but plugins now extend
        // the core functionality rather than provide it
        let name = plugin.name().to_string();
        
        // Build the plugin (allow it to extend the app)
        plugin.build(self);
        
        // Store the plugin
        self.plugins.insert(name, Box::new(plugin));
        
        self
    }
}
```

### App Initialization with Features

```rust
impl App {
    // Optional methods for configuring renderer
    #[cfg(feature = "render")]
    pub fn with_window(mut self, window_config: WindowConfig) -> Self {
        self.renderer = Some(Renderer::new(window_config));
        self
    }
    
    // Builder pattern for configuration
    pub fn with_max_entities(mut self, max_entities: usize) -> Self {
        self.world = World::with_capacity(max_entities);
        self
    }
}
```

## ECS Implementation: rustica_ecs

### Remove Plugin Dependencies

```rust
// Remove EcsPlugin or make it optional
#[deprecated(
    since = "0.2.0",
    note = "ECS is now a core component. Use App::new() instead of adding this plugin."
)]
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        // Do nothing - core components are already initialized
        // This exists only for backward compatibility
    }
}
```

### Direct World Integration

```rust
// World should be usable directly, not just through plugins
impl World {
    // Direct entity creation methods
    pub fn spawn(&mut self) -> EntityBuilder {
        // Entity creation logic
    }
    
    // Direct component registration
    pub fn register_component<T: Component>(&mut self) {
        // Component registration logic
    }
}
```

## Scheduler Implementation: rustica_scheduler

### Remove Plugin Dependencies

```rust
#[deprecated(
    since = "0.2.0",
    note = "Scheduler is now a core component. Use App::new() instead of adding this plugin."
)]
pub struct SchedulerPlugin;

impl Plugin for SchedulerPlugin {
    fn build(&self, app: &mut App) {
        // Do nothing - core components are already initialized
        // This exists only for backward compatibility
    }
}
```

### Direct System Registration

```rust
// Schedule should be usable directly
impl Schedule {
    // Direct system registration methods that App can call
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> Result<(), SchedulerError>
    where
        S: System,
    {
        // System registration logic
    }
}
```

## Event System Implementation: rustica_event

### Direct Event Integration

```rust
// EventSystem as a core component
pub struct EventSystem {
    // Event queues and handlers
    event_queues: HashMap<TypeId, Box<dyn Any>>,
    event_handlers: HashMap<TypeId, Vec<Box<dyn Fn(&mut World, &dyn Any)>>>,
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            event_queues: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }
    
    // Register event types
    pub fn register_event<E: Event>(&mut self) {
        // Event registration logic
    }
    
    // Add event handlers
    pub fn add_event_handler<E: Event>(&mut self, handler: impl Fn(&mut World, &E) + 'static) {
        // Handler registration logic
    }
    
    // Send events
    pub fn send_event<E: Event>(&mut self, event: E) {
        // Event sending logic
    }
    
    // Process queued events
    pub fn process_events(&mut self, world: &mut World) {
        // Event processing logic
    }
}
```

## Renderer Implementation: rustica_render

### Direct Renderer Integration

```rust
// Only when render feature is enabled
#[cfg(feature = "render")]
pub struct Renderer {
    window: Window,
    // Other rendering state
}

#[cfg(feature = "render")]
impl Renderer {
    pub fn new(config: WindowConfig) -> Self {
        // Initialize renderer
    }
    
    pub fn render(&mut self, world: &World) {
        // Rendering logic
    }
}
```

## Migration Path

To ensure backward compatibility during the transition, we'll:

1. Implement direct component integration
2. Keep existing plugins but make them no-ops with deprecation notices
3. Update examples to demonstrate both approaches
4. Create documentation with migration guides

## Example App Implementation (After Refactoring)

```rust
use rustica::prelude::*;

fn main() {
    // Create app with core components already initialized
    let mut app = App::new()
        .with_window(WindowConfig {
            title: "My Game".to_string(),
            width: 800,
            height: 600,
        })
        .with_max_entities(10000);
    
    // Register game components directly
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
    
    // Add game systems directly
    app.add_system(movement_system, "movement", Stage::Update);
    
    // Add true extension plugins
    app.add_plugin(MyGamePlugin);
    
    app.run();
}

// Component definitions
struct Position(f32, f32);
struct Velocity(f32, f32);

// System implementations
fn movement_system(world: &mut World) {
    // System logic
}

// Game plugin for extensions beyond core functionality
struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // Configure gameplay-specific features
        app.add_system(player_input_system, "player_input", Stage::PreUpdate);
        
        // Spawn initial entities
        let entity = app.world_mut().spawn()
            .with(Position(0.0, 0.0))
            .with(Velocity(1.0, 0.0))
            .build();
    }
}

fn player_input_system(world: &mut World) {
    // System logic
}
```

## Testing Strategy

Each refactored component needs unit tests to ensure it works correctly both standalone and integrated:

1. Test core components individually
2. Test integration between components
3. Test backward compatibility with plugin system
4. Test examples with new approach

## Implementation Sequence

1. Start with `rustica_core` to establish the foundation
2. Refactor `rustica_ecs` to work directly with App
3. Update `rustica_scheduler` for direct integration
4. Modify `rustica_event` to become a core component
5. Adjust `rustica_render` for direct integration when enabled
6. Update the main `rustica` facade to reflect new architecture
7. Update examples and documentation
