# Rustica Engine

A modular 3D graphics and game engine written in Rust.

## Overview

Rustica is a modern, modular 3D graphics and game engine designed with a focus on simplicity, performance, and extensibility. Built entirely in Rust, it leverages the language's safety guarantees and performance characteristics to provide a robust foundation for graphics applications and games.

## Architecture

Rustica is organized into several modular crates, each with a specific responsibility:

- **Foundation**: Core geometric primitives and data structures
- **Render**: High-level rendering API built on wgpu
- **Graphics**: Higher-level graphics primitives and utilities
- **Window**: Window management and event handling
- **Core**: Central engine component (in development)
- **Extensions**: Optional engine extensions (in development)
- **Render-Derive**: Procedural macros for the render crate

## Features

- **Modern Graphics API**: Built on wgpu for cross-platform Vulkan, Metal, DX12, and WebGPU support
- **Modular Design**: Use only the components you need
- **Custom Shaders**: Support for custom WGSL shaders
- **Mesh System**: Flexible mesh representation and rendering
- **Camera System**: Perspective and orthographic cameras
- **Primitive Generation**: Built-in primitives like UV spheres

## Getting Started

### Prerequisites

- Rust 1.70 or later
- A GPU that supports Vulkan, Metal, or DX12

### Installation

Add Rustica to your project by including the crates you need in your `Cargo.toml`:

```toml
[dependencies]
rustica_foundation = { path = "path/to/rustica/Foundation" }
rustica_render = { path = "path/to/rustica/crates/render" }
rustica_graphics = { path = "path/to/rustica/crates/graphics" }
rustica_window = { path = "path/to/rustica/crates/window" }
```

### Basic Example

```rust
use rustica_render::RenderWindow;
use rustica_graphics::primitives::sphere::create_default_sphere;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a render window
    let mut render_window = RenderWindow::new("Rustica Example", 800, 600)?;
    
    // Create a sphere
    let sphere = create_default_sphere();
    
    // Register the mesh with the render window
    let mesh_id = render_window.register_mesh(&sphere)?;
    
    // Run the render loop
    render_window.run(move |canvas| {
        // Clear the canvas
        canvas.clear([0.1, 0.2, 0.3, 1.0]);
        
        // Draw the sphere
        canvas.draw_mesh(mesh_id);
        
        Ok(())
    })?;
    
    Ok(())
}
```

## Development Approach

Rustica is developed using a prototype-first approach:

1. Create a prototype to prove a concept
2. Extract the functionality into the appropriate crate
3. Update the prototype to use the crate
4. Repeat for the next feature

This approach ensures that each component is thoroughly tested and proven before being integrated into the main engine.

## Project Structure

- **Foundation/**: Core geometric primitives
- **crates/**: Main engine crates
  - **render/**: Rendering functionality
  - **graphics/**: Graphics primitives
  - **window/**: Window management
  - **render-derive/**: Procedural macros
- **Core/**: Central engine component (in development)
- **Extensions/**: Optional engine extensions (in development)
- **Prototypes/**: Proof-of-concept implementations
- **Examples/**: Example applications

## License

This project is licensed under the MIT License - see the LICENSE file for details.
