# Render Pipeline Design

## Overview

The render pipeline in Rustica follows a clear separation of concerns between game state and rendering. This architecture ensures that game objects (like stars) have no knowledge of how they are rendered, and the renderer has no knowledge of specific game objects.

```
Game State → ViewModel → Render Elements → GPU Batches
```

## Components

### 1. Game State (ECS Components)

Game objects are defined by components that describe their properties:

```rust
// Star's game properties - knows nothing about rendering
struct Star {
    brightness: f32,
    size: f32,
    color: Option<[f32; 3]>,
}

// Position component - pure data, no rendering knowledge
struct Position {
    value: Vec3<f32>,
}
```

### 2. Render Descriptors (DebugComponents)

Components that describe how an entity should be rendered:

```rust
// Describes how to render a point/star
struct DebugStarComponent {
    color: Vec4<f32>,
    size: f32,
    brightness: f32,
    visible: bool,
}
```

These components are attached to entities alongside game state components.

### 3. Render Commands (ViewModel Layer)

Generic, renderer-agnostic drawing commands that describe what to draw:

```rust
// A list of things to draw
struct RenderCommandList {
    point_commands: Vec<DrawPointCommand>,
    line_commands: Vec<DrawLineCommand>,
    rect_commands: Vec<DrawRectCommand>,
}

// Command to draw a point at a position
struct DrawPointCommand {
    position: Vec3<f32>,
    color: Vec4<f32>,
    size: f32,
    brightness: f32,
}
```

These commands know nothing about stars or other game objects - they just describe points, lines, and shapes to draw.

### 4. Rendering System

The ECS system that bridges game state and rendering:

```rust
// Queries for components and generates render commands
fn debug_star_render_system(world: &mut World) {
    // Get or create the command list
    let mut cmd_list = get_command_list(world);
    
    // Clear previous commands
    cmd_list.clear();
    
    // Query for entities with Position and DebugStarComponent
    for (entity, position, debug_star) in query_components(world) {
        if debug_star.visible {
            // Add render command (no knowledge of "stars")
            cmd_list.add_point(
                position.value,
                debug_star.color,
                debug_star.size,
                debug_star.brightness
            );
        }
    }
}

// Processes render commands into GPU draw calls
fn debug_render_process_system(world: &mut World) {
    // Get renderer and command list
    let renderer = get_renderer(world);
    let cmd_list = get_command_list(world);
    
    // Begin frame
    let frame = renderer.begin_frame();
    
    // Process all point commands
    for cmd in &cmd_list.point_commands {
        // Draw using renderer's primitive operations
        renderer.draw_point(cmd); 
    }
    
    // End frame
    renderer.end_frame(frame);
}
```

## Flow

1. Game code creates entities with game state components (`Star`, `Position`) and render descriptor components (`DebugStarComponent`)
2. Rendering system queries for entities with these components
3. Rendering system generates generic render commands from the descriptors
4. Rendering system processes these commands into GPU draw calls

## Benefits

1. **Separation of Concerns**:
   - Game code knows nothing about rendering details
   - Renderer knows nothing about game objects
   - ECS provides the bridge between them

2. **Optimization Opportunities**:
   - Early culling in command generation phase
   - Batching of similar draw calls
   - Caching of intermediary results

3. **Flexibility**:
   - Any entity can be rendered by attaching the right components
   - Renderer can be swapped out without changing game code
   - New renderable types can be added without changing existing code
