# Rustica Architecture Diagrams

This document presents visual diagrams of the current architecture and the proposed refactored architecture.

## Current Architecture

The current architecture implements everything as plugins, including core engine functionality:

```mermaid
graph TD
    A[App] --> B[Plugin Registry]
    B --> C1[EcsPlugin]
    B --> C2[SchedulerPlugin]
    B --> C3[RenderPlugin]
    B --> C4[EventPlugin]
    B --> C5[Game Plugins]
    
    C1 --> D1[World Resource]
    C1 --> D2[Time Resource]
    
    C2 --> E1[Schedule Resource]
    C2 --> E2[System Management]
    
    C3 --> F1[Window Resource]
    C3 --> F2[Renderer Resource]
    
    C4 --> G1[Event Queues]
    
    A --> H[Resource Registry]
    H --> D1
    H --> D2
    H --> E1
    H --> F1
    H --> F2
    H --> G1
    
    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C1 fill:#dfd,stroke:#333,stroke-width:2px
    style C2 fill:#dfd,stroke:#333,stroke-width:2px
    style C3 fill:#dfd,stroke:#333,stroke-width:2px
    style C4 fill:#dfd,stroke:#333,stroke-width:2px
    style C5 fill:#fdb,stroke:#333,stroke-width:2px
```

### Current Issues

1. **Excessive Indirection**: Core functionality hidden behind plugins
2. **Circular Dependencies**: Plugins needing to know about each other
3. **Unclear Responsibilities**: No distinction between core and extension plugins
4. **Resource Management Complexity**: Components registered as resources

## Proposed Architecture

The refactored architecture directly incorporates core systems while keeping the plugin system for extensions:

```mermaid
graph TD
    A[App] --> B1[World]
    A --> B2[Schedule]
    A --> B3[EventSystem]
    A --> B4[Renderer]
    A --> C[Plugin Registry]
    
    B1 --> D1[Entity Management]
    B1 --> D2[Component Storage]
    
    B2 --> E1[System Management]
    B2 --> E2[Stage Execution]
    
    B3 --> F1[Event Queues]
    B3 --> F2[Event Handlers]
    
    B4 --> G1[Window]
    B4 --> G2[Rendering Pipeline]
    
    C --> H1[Game Plugin]
    C --> H2[UI Plugin]
    C --> H3[Custom System Plugins]
    
    H1 --> I1[Game Components]
    H1 --> I2[Game Systems]
    
    A --> J[Resource Registry]
    
    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B1 fill:#dfd,stroke:#333,stroke-width:2px
    style B2 fill:#dfd,stroke:#333,stroke-width:2px
    style B3 fill:#dfd,stroke:#333,stroke-width:2px
    style B4 fill:#dfd,stroke:#333,stroke-width:2px
    style C fill:#bbf,stroke:#333,stroke-width:2px
    style H1 fill:#fdb,stroke:#333,stroke-width:2px
    style H2 fill:#fdb,stroke:#333,stroke-width:2px
    style H3 fill:#fdb,stroke:#333,stroke-width:2px
```

### Key Improvements

1. **Direct Access**: Core systems directly accessible as App fields
2. **Clear Boundaries**: Distinction between engine fundamentals and plugins
3. **Simplified API**: Direct methods for common operations
4. **Better Performance**: Reduced indirection and overhead
5. **Focused Plugin System**: Plugins used only for true extensions

## API Comparison

### Before Refactoring

```rust
// Initialize app with plugins
let mut app = App::new();
app.add_plugin(EcsPlugin::default());
app.add_plugin(SchedulerPlugin::default());
app.add_plugin(RenderPlugin::default());
app.add_plugin(EventPlugin::default());

// Add game-specific plugin
app.add_plugin(MyGamePlugin);
```

### After Refactoring

```rust
// Core systems initialized automatically
let mut app = App::new();

// Configure core systems directly
app.register_component::<Position>();
app.add_system(move_system, "move", Stage::Update);

// Add only game-specific plugins
app.add_plugin(MyGamePlugin);
```

## Component Relationships

```mermaid
classDiagram
    class App {
        +World world
        +Schedule schedule
        +EventSystem eventSystem
        +Renderer? renderer
        +HashMap~String,BoxedPlugin~ plugins
        +HashMap~TypeId,BoxedResource~ resources
        +bool running
        +new() App
        +add_plugin(Plugin) App
        +world() World
        +world_mut() World
        +add_system(System, name, stage) App
        +register_component~T~() App
        +register_event~E~() App
        +update() void
        +run() void
    }
    
    class World {
        +new() World
        +spawn() EntityBuilder
        +register_component~T~() void
        +query~T~() Query
    }
    
    class Schedule {
        +new() Schedule
        +add_system(System, name, stage) Result
        +run(World) void
    }
    
    class EventSystem {
        +new() EventSystem
        +register_event~E~() void
        +send_event~E~(event) void
        +process_events(World) void
    }
    
    class Plugin {
        +build(App) void
    }
    
    App *-- World : contains
    App *-- Schedule : contains
    App *-- EventSystem : contains
    App *-- Plugin : registers
    App o-- "0..1" Renderer : optional
    
    class GamePlugin {
        +build(App) void
    }
    
    Plugin <|-- GamePlugin : implements
```

## File Organization

```
crates/
├── rustica/           # Main facade/prelude
├── rustica_core/      # Core App + Lifecycle
│   ├── app.rs         # App struct with direct core systems
│   └── plugin.rs      # Plugin trait for extensions
├── rustica_ecs/       # ECS implementation
│   ├── world.rs       # World implementation
│   └── component.rs   # Component trait
├── rustica_scheduler/ # Scheduler implementation
│   ├── schedule.rs    # Schedule implementation
│   └── system.rs      # System trait
├── rustica_event/     # Event system
├── rustica_render/    # Rendering (optional)
└── rustica_common/    # Shared utilities
