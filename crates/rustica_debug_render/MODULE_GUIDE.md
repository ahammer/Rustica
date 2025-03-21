# Rustica Debug Renderer Module Guide

## Overview

The `rustica_debug_render` crate provides a simple but effective debug rendering system built on wgpu. It allows for visualizing game objects and debugging game state with minimal performance overhead.

## Core Components

### `DebugRenderer`

The central component of this crate is the `DebugRenderer` struct, which manages the wgpu device, surface, and pipelines. It provides methods for drawing basic primitives and integrates with the window system.

### `PrimitiveRenderer`

Handles the rendering of basic 2D primitives like points, lines, and rectangles. These are essential for debugging game state and visualizing collision shapes, paths, etc.

## Key Modules

### `debug_renderer.rs`

Contains the core `DebugRenderer` struct and implementation. This handles:
- Initialization of wgpu device, queue, and surface
- Managing render pipelines and bind groups
- Frame lifecycle (begin_frame, end_frame)
- Integration with the window system

### `primitives.rs`

Defines basic 2D rendering primitives:
- Point rendering
- Line rendering
- Rectangle rendering
- Text rendering (future implementation)

### `star_renderer.rs`

Specialized renderer for star fields:
- Batching system for efficient star rendering
- Different star rendering modes (point, sprite)
- Color and brightness variations

### `plugin.rs`

Implements the `DebugRenderPlugin` for integration with the Rustica engine:
- Registers systems for debug rendering
- Sets up resources like the renderer
- Connects to the window system

## Usage

### Basic Debug Rendering

```rust
// Initialize the debug renderer
let mut debug_renderer = DebugRenderer::new(&window)?;

// In render system
debug_renderer.begin_frame()?;

// Draw primitives
debug_renderer.draw_point(Vec2::new(100.0, 100.0), 5.0, Vec4::new(1.0, 0.0, 0.0, 1.0)); // Red point
debug_renderer.draw_line(
    Vec2::new(50.0, 50.0), 
    Vec2::new(150.0, 150.0), 
    2.0, 
    Vec4::new(0.0, 1.0, 0.0, 1.0)
); // Green line
debug_renderer.draw_rect(
    Vec2::new(200.0, 200.0), 
    Vec2::new(50.0, 50.0), 
    Vec4::new(0.0, 0.0, 1.0, 1.0)
); // Blue rectangle

debug_renderer.end_frame()?;
```

### Star Rendering

```rust
// Initialize the star renderer
let mut star_renderer = StarRenderer::new(&debug_renderer)?;

// Add stars
for star in stars {
    star_renderer.add_star(
        star.position, 
        star.size, 
        star.color, 
        star.brightness
    );
}

// Render stars
star_renderer.render(&debug_renderer)?;
```

## Integration with ECS

The debug renderer integrates with the ECS by:
1. Providing rendering systems that query for renderable components
2. Supporting visualization of entity positions and other debug information
3. Offering a way to visualize the world state during development

## Future Enhancements

- Text rendering for debug information
- Shape rendering for complex collision shapes
- Debug cameras for different perspectives
- Performance visualization tools
- Debug UI elements
