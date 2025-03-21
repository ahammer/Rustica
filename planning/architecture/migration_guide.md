# Rustica Migration Guide

This document provides guidance for migrating existing code to the new architecture. It outlines the changes that users of the Rustica engine will need to make to update their code to work with the refactored architecture.

## For Engine Users

### Current Usage Pattern

```rust
// Initialize app with all required plugins
let mut app = App::new();

// Add core functionality plugins
app.add_plugin(EcsPlugin::default());
app.add_plugin(SchedulerPlugin::default());
app.add_plugin(EventPlugin::default());

#[cfg(feature = "render")]
app.add_plugin(RenderPlugin::default());

// Add game-specific plugins
app.add_plugin(MyGamePlugin);

// Run the application
app.run();
```

### New Usage Pattern

```rust
// Core systems are automatically initialized
let mut app = App::new();

// Optional: Configure a window if needed
#[cfg(feature = "render")]
let app = app.with_window(WindowConfig {
    title: "My Game",
    width: 800,
    height: 600,
});

// Add only game-specific plugins
app.add_plugin(MyGamePlugin);

// Run the application
app.run();
```

### Migration Steps

1. **Remove Core Plugin Registration**
   - Remove calls to add `EcsPlugin`, `SchedulerPlugin`, `EventPlugin`, and `RenderPlugin`
   - These systems are now automatically initialized in `App::new()`

2. **Update Window Configuration**
   - Replace `RenderPlugin` configuration with the builder method `with_window`
   - Move any renderer configuration from plugin to the configuration method

3. **Use Direct Registration Methods**
   - Replace plugin-specific registration methods with App methods
   - For example, use `app.register_component<T>()` instead of plugin-specific methods

4. **Update Resource Management**
   - Continue using `insert_resource` as before, but prefer direct field access for core systems
   - For example, use `app.world_mut()` instead of getting the world as a resource

## For Game Plugin Authors

### Current Plugin Implementation

```rust
struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // Get core systems as resources
        let world = app.get_resource_mut::<World>().unwrap();
        let schedule = app.get_resource_mut::<Schedule>().unwrap();
        
        // Register components
        world.register_component::<Position>();
        
        // Add systems
        schedule.add_system(move_system, "move", Stage::Update);
        
        // Initialize game state
        // ...
    }
}
```

### New Plugin Implementation

```rust
struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // Access core systems directly
        app.register_component::<Position>();
        app.add_system(move_system, "move", Stage::Update);
        
        // Spawn entities
        app.world_mut().spawn()
            .add_component(Position::new(0.0, 0.0))
            .build();
            
        // Initialize game state
        // ...
    }
}
```

### Migration Steps

1. **Update System Access**
   - Replace resource access with direct methods
   - Use `app.world()` instead of `app.get_resource::<World>()`
   - Use `app.add_system()` instead of accessing the schedule and adding systems

2. **Use Convenience Methods**
   - Take advantage of new convenience methods on `App`
   - For example, use `app.register_component<T>()` instead of accessing world first
   - Use `app.register_event<E>()` instead of working with the event system directly

3. **Simplify Entity Creation**
   - Use `app.world_mut().spawn()` for direct entity creation
   - Chain component adding operations for clearer entity setup

## For Engine Contributors

### Current Structure

```rust
// Component crates provide plugins
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::new());
        // ...
    }
}

// App orchestrates via plugins
pub struct App {
    plugins: HashMap<String, BoxedPlugin>,
    resources: HashMap<TypeId, BoxedResource>,
    // ...
}
```

### New Structure

```rust
// App directly contains core systems
pub struct App {
    world: World,
    schedule: Schedule,
    event_system: EventSystem,
    // ...
    
    plugins: HashMap<String, BoxedPlugin>,
    resources: HashMap<TypeId, BoxedResource>,
}

// Component crates optionally provide backward compatibility
#[deprecated]
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, _app: &mut App) {
        // Do nothing - App already has World
    }
}
```

### Migration Steps

1. **Update Core Systems**
   - Move core system implementations to direct field access instead of plugins
   - Add direct access methods for all core systems
   - Ensure backward compatibility traits are implemented

2. **Maintain Deprecated Plugins**
   - Keep old plugin implementations but mark as deprecated
   - Make them no-ops that don't modify the app

3. **Update Integration Tests**
   - Update integration tests to use the new direct App methods
   - Ensure existing plugin-based tests still work through compatibility

## Handling Circular Dependencies

The new architecture reduces circular dependencies by making the core systems part of the App structure directly. For any remaining dependencies between systems:

1. **Use Trait Objects**
   - Define traits for system-to-system communication
   - Use trait objects to break circular dependencies

2. **Event-Based Communication**
   - Use the event system for cross-system communication
   - Systems can communicate without direct knowledge of each other

3. **Dependency Injection Pattern**
   - Systems that need access to other systems can take references
   - Configure dependencies at App initialization time

## Support Timeline

- **v0.2.0**: Introduce new architecture with backward compatibility
- **v0.3.0**: Mark old plugin-based approach as deprecated
- **v1.0.0**: Remove deprecated plugin implementations

## Common Issues and Solutions

### Issue: "Cannot find EcsPlugin/SchedulerPlugin/etc."

**Solution**: These plugins are no longer needed. Remove them from your code and use the core systems directly through App methods.

### Issue: "How do I configure the renderer?"

**Solution**: Use the builder pattern: `app.with_window(config)` instead of adding RenderPlugin.

### Issue: "System doesn't have access to components"

**Solution**: Ensure components are registered with `app.register_component::<T>()` before systems are added.

### Issue: "Plugin dependency validation fails"

**Solution**: Update plugin dependencies to reflect the new architecture. Core systems no longer need to be listed as dependencies.
