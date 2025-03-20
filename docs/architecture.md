# Rustica Game Engine Architecture

This document outlines the high-level architecture of the Rustica game engine, describing the relationships between components and the design philosophy.

## Architecture Principles

1. **Minimal Core**: The core module provides only orchestration and plugin management.
2. **Modular Design**: All functionality is implemented in independent subsystems.
3. **Clean Interfaces**: Each module has a clear public API boundary.
4. **Plugin-Based Extension**: New functionality is added through the plugin system.
5. **Data-Oriented Design**: Focus on data layouts and transformations over object hierarchies.

## System Overview

```
┌───────────────────────────────────────────────────────────────┐
│                        Rustica (Facade)                        │
└───────────────────────────────────────────────────────────────┘
                  │                │                │
┌─────────────────▼─────┐ ┌────────▼────────┐ ┌────▼─────────────┐
│    Rustica Core        │ │  Subsystems     │ │  User Plugins    │
│  ┌─────────────────┐   │ │ ┌─────────────┐ │ │ ┌──────────────┐ │
│  │ Plugin Registry │◄──┼─┼─┤ ECS Plugin  │ │ │ │ Game Plugin  │ │
│  └─────────────────┘   │ │ └─────────────┘ │ │ └──────────────┘ │
│  ┌─────────────────┐   │ │ ┌─────────────┐ │ │ ┌──────────────┐ │
│  │   App Builder   │◄──┼─┼─┤Event Plugin │ │ │ │Custom Systems│ │
│  └─────────────────┘   │ │ └─────────────┘ │ │ └──────────────┘ │
│  ┌─────────────────┐   │ │ ┌─────────────┐ │ │        ▲        │
│  │ Lifecycle Mgmt  │◄──┼─┼─┤Render Plugin│ │ │        │        │
│  └─────────────────┘   │ │ └─────────────┘ │ │        │        │
└─────────────────────────┘ └────────────────┘ └────────┼─────────┘
                                 ▲                      │
                                 └──────────────────────┘
```

## Component Responsibilities

### Rustica Facade (crates/rustica)

The main entry point that re-exports public APIs from all subsystems. Provides:
- A unified prelude for easy imports
- Version alignment across all crates
- Public API documentation

### Rustica Core (crates/rustica_core)

The minimal orchestration layer that:
- Defines the Plugin trait
- Manages the App lifecycle
- Handles plugin registration and initialization
- Provides the main run loop

### ECS Subsystem (crates/rustica_ecs)

Entity-Component-System implementation that:
- Manages entity creation and destruction
- Stores component data
- Provides query mechanisms for accessing entities and components

### Event System (crates/rustica_event)

Handles communication between subsystems:
- Event creation and dispatch
- Subscription mechanisms
- Event queues and processing

### Scheduler (crates/rustica_scheduler)

Manages system execution:
- System registration and scheduling
- Parallel execution
- System dependencies and ordering

### Rendering (crates/rustica_render)

Graphics abstraction layer:
- Window management
- Rendering interface
- Graphics resource management
- Support for headless testing

### Math Utilities (crates/rustica_math)

Core mathematical types and operations:
- Vector and matrix math
- Transformations
- Utility functions

## Data Flow

1. The App is initialized and plugins are registered
2. Systems are added to the scheduler by plugins
3. The main loop runs, calling into the scheduler
4. Systems operate on components through queries
5. Events are dispatched between systems for communication
6. Rendering occurs at the end of each frame

## Plugin System

The plugin system is the primary extension mechanism:

```rust
pub trait Plugin {
    fn build(&self, app: &mut App);
}
```

Plugins can:
- Register components
- Add systems to the scheduler
- Provide resources
- Register event types
- Add rendering capabilities

## Example Flow

```
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐
│ App::new()|    │add_plugin │    │add_plugin │    │   run()   │
│           ├───►│ (Core)    ├───►│(Renderer) ├───►│           │
└───────────┘    └───────────┘    └───────────┘    └─────┬─────┘
                                                         │
┌───────────┐    ┌───────────┐    ┌───────────┐    ┌─────▼─────┐
│Event Loop │    │  Update   │    │  Render   │    │Initialize │
│           │◄───┤  Systems  │◄───┤  Systems  │◄───┤           │
└─────┬─────┘    └───────────┘    └───────────┘    └───────────┘
      │
      └──── (Repeat until exit)
```

## Cross-Cutting Concerns

### Error Handling

- All fallible operations return Result<T, Error>
- Errors are propagated up to appropriate boundaries
- Fatal errors terminate the application
- Non-fatal errors are logged and handled gracefully

### Resource Management

- Assets and resources are loaded asynchronously
- Resource lifetime is managed by the ECS
- Resources are accessed through queries or resource handles

### Testing

- All subsystems can be tested independently
- Headless testing for graphical components
- Mock implementations for testing interactions
