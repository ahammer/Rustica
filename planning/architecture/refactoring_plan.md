# Rustica Architecture Refactoring Plan

## Current Issues

The current architecture of Rustica has several issues that need to be addressed:

1. **Over-reliance on plugins**: Everything is implemented as a plugin, including core functionality like ECS, Scheduler, and Renderer.
2. **Complexity overhead**: The plugin system creates indirection and complexity for essential components.
3. **Unclear boundaries**: There's no clear distinction between engine fundamentals and extension points.
4. **Circular dependencies**: Core components depending on each other through the plugin system.
5. **Configuration overhead**: Simple engine setups require excessive plugin wiring.

## Architectural Vision

We propose a clearer separation of concerns with three distinct layers:

```
┌──────────────────────────────────────────────────────────────────────┐
│                    User Game Code / Applications                      │
└────────────────────────────────┬─────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                           Plugin Layer                               │
│                                                                      │
│  Game-specific extensions, custom systems, UI components, etc.       │
│  Uses Plugin trait for integration                                   │
└────────────────────────────────┬─────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                           Engine Layer                               │
│                                                                      │
│  ECS, Scheduler, Renderer, Event System                              │
│  Direct parts of App, not plugins                                    │
└────────────────────────────────┬─────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────┐
│                            Core Layer                                │
│                                                                      │
│  App lifecycle, plugin management, minimalist orchestration          │
└──────────────────────────────────────────────────────────────────────┘
```

### 1. Core Layer
- **Purpose**: Minimal application lifecycle, plugin orchestration
- **Components**: App struct, Plugin trait, error handling
- **Characteristics**: No implementation-specific dependencies

### 2. Engine Layer
- **Purpose**: Provide fundamental engine capabilities
- **Components**: ECS, Scheduler, Renderer, Event System
- **Characteristics**: Direct members of App, initialized in App::new() or via builder pattern

### 3. Plugin Layer
- **Purpose**: Enable game-specific content and extensions
- **Components**: Game systems, UI components, custom rendering, etc.
- **Characteristics**: Truly optional, focused on user customization

## Implementation Strategy

### Phase 1: Refactor Core Components

1. **App Structure Refactoring**
   - Move core engine systems from plugins to direct members of the App struct
   - Add initialization in App::new() or provide a builder pattern
   - Ensure App directly manages the lifecycle of these components

2. **Define Core System Interfaces**
   - Establish clean interfaces for core systems (ECS, Scheduler, etc.)
   - Focus on decoupling implementations while maintaining integration

3. **Dependency Management**
   - Remove circular dependencies between core systems
   - Use trait objects or type erasure where needed for loose coupling

### Phase 2: Plugin System Refinement

1. **Refocus Plugin Trait**
   - Simplify the Plugin trait to focus on extending engine functionality
   - Provide clear documentation about appropriate plugin use cases

2. **Establish Extension Points**
   - Define clear extension points for plugins in core systems
   - Document how to extend without deep engine knowledge

3. **Simplified Integration**
   - Create direct methods for common operations
   - Remove need for plugin system for basic engine functionality

### Phase 3: API Cleanup and Documentation

1. **Streamline Public API**
   - Clean up public exports to match new architecture
   - Ensure prelude modules provide the most commonly used types

2. **Update Documentation**
   - Revise architecture documentation
   - Update examples to demonstrate the new approach
   - Add migration guides for existing code

## Specific Crate Changes

### rustica_core

- Remove dependencies on implementation-specific plugins
- Add direct fields for core systems (World, Schedule, etc.)
- Simplified App initialization with core systems
- Keep Plugin trait for true extension points

### rustica_ecs

- Move from plugin-based integration to direct integration with App
- Expose clean interface for plugins to register components/systems
- Remove EcsPlugin or make it optional for backward compatibility

### rustica_scheduler

- Integrate directly with App instead of through plugin system
- Provide direct schedule/system registration methods
- Remove SchedulerPlugin or make it optional

### rustica_render

- Move core rendering capability to direct App integration
- Keep plugin system for rendering extensions/effects
- Simplify window/renderer initialization

### rustica_event

- Direct integration with App
- Clean API for event registration and dispatching
- Remove plugin-based integration

### rustica (main facade)

- Update re-exports to match new architecture
- Streamline prelude to focus on common use cases
- Ensure examples demonstrate the simplified approach

## Code Examples

### Before Refactoring

```rust
fn main() {
    let mut app = App::new();
    
    // Add required plugins
    app.add_plugin(EcsPlugin::default());
    app.add_plugin(SchedulerPlugin::default());
    app.add_plugin(RenderPlugin::default()); 
    app.add_plugin(EventPlugin::default());
    
    // Add game plugin
    app.add_plugin(MyGamePlugin);
    
    app.run();
}
```

### After Refactoring

```rust
fn main() {
    // Core systems automatically initialized
    let mut app = App::new();
    
    // Only add actual game plugins
    app.add_plugin(MyGamePlugin);
    
    app.run();
}
```

## Migration Strategy

1. Keep backward compatibility during transition
2. Introduce new API alongside existing plugin-based approach
3. Mark old APIs as deprecated with clear migration paths
4. Provide detailed examples for both approaches during transition
5. Complete removal of redundant plugin systems in a future major version

## Implementation Timeline

1. **Immediate**: Documentation updates and architectural planning
2. **Short-term**: Core system direct integration with backward compatibility
3. **Mid-term**: Plugin system refocusing and extension point definition
4. **Long-term**: Complete API cleanup and removal of redundant patterns

## Conclusion

This refactoring will significantly simplify the architecture of Rustica while maintaining its extensibility. By clearly separating core engine functionality from plugin-based extensions, we'll reduce complexity, improve performance, and provide a better developer experience for both engine users and contributors.
