# Rustica Render Derive

The Render Derive crate provides procedural macros for the Rustica engine.

## Overview

Rustica Render Derive provides derive macros for the `rustica_render` crate, making it easier to work with custom vertex types and shaders. These macros automate the implementation of traits and generate boilerplate code, reducing the amount of manual code required.

## Vertex Derive Macro

The `Vertex` derive macro automatically implements the `Vertex` trait for your custom vertex types, generating the appropriate vertex buffer layout based on the struct fields. It also automatically implements `bytemuck::Pod` and `bytemuck::Zeroable` traits, which are required for GPU data transfer.

### Features

- **Automatic shader location assignment**: Fields are assigned shader locations (0, 1, 2, etc.) in the order they are declared in the struct.
- **Format detection**: The macro automatically detects the appropriate format based on the field type (e.g., `[f32; 3]` → `Float32x3`).
- **Custom attributes**: You can customize the behavior using attributes.
- **Automatic Pod and Zeroable implementation**: The macro automatically implements `bytemuck::Pod` and `bytemuck::Zeroable` traits.

### Usage

```rust
use rustica_render::Vertex;

// Basic usage - automatic location assignment and format detection
// Note: Pod and Zeroable are automatically implemented
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct SimpleVertex {
    position: [f32; 3], // location = 0, format = Float32x3
    color: [f32; 3],    // location = 1, format = Float32x3
    uv: [f32; 2],       // location = 2, format = Float32x2
}

// Advanced usage - custom locations and formats
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CustomVertex {
    #[vertex(location = 2)] // Override the location
    position: [f32; 3],
    
    #[vertex(location = 0, format = "Float32x4")] // Override both location and format
    color: [f32; 3],
    
    #[vertex(skip)] // Skip this field in the vertex layout
    _padding: u32,
    
    uv: [f32; 2], // Automatic location and format
}
```

> **Note**: The `#[repr(C)]` attribute is still required to ensure a consistent memory layout compatible with GPU expectations.

### Attributes

The `vertex` attribute supports the following parameters:

- `location`: Override the shader location for this field.
- `format`: Override the format for this field (e.g., `"Float32x3"`, `"Uint32x2"`).
- `skip`: Skip this field in the vertex layout.

## ShaderDescriptor Derive Macro

The `ShaderDescriptor` derive macro automatically creates a shader descriptor from a struct that references a vertex type and uniform parameters.

### Features

- **Automatic vertex attribute extraction**: The macro extracts vertex attributes from the referenced vertex type.
- **Automatic uniform parameter handling**: Fields marked with `#[uniform]` are automatically added as uniform parameters.
- **Shader source loading**: The macro can load shader source from a file or use inline shader code.

### Usage

```rust
use rustica_render::{ShaderDescriptor, Vertex};

// Define your vertex type
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CustomVertex {
    position: [f32; 3],
    color: [f32; 3],
    uv: [f32; 2],
}

// Define your shader descriptor
#[allow(dead_code)] // Suppress unused field warnings
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/my_shader.wgsl")] // Path to shader file
struct MyShaderDescriptor {
    #[vertex_type]
    vertex: CustomVertex, // Reference to your vertex type
    
    #[uniform(binding = 0)]
    time: f32, // Uniform parameter
    
    #[uniform(binding = 1)]
    resolution: [f32; 2], // Another uniform parameter
}

// Use it in your code
fn main() {
    // Create the shader descriptor
    let shader_descriptor = MyShaderDescriptor::descriptor();
    
    // Use it with the render system
    let shader_id = render_window.register_shader(shader_descriptor);
}
```

### Attributes

The `ShaderDescriptor` derive macro supports the following attributes:

- `#[shader(source = "path/to/shader.wgsl")]`: Specify the path to the shader source file.
- `#[shader(inline = "shader source code")]`: Provide inline shader source code.
- `#[vertex_type]`: Mark a field as the vertex type reference.
- `#[uniform(binding = N)]`: Mark a field as a uniform parameter with the specified binding index.

## Integration with Rustica Render

To use these derive macros, add the following to your `Cargo.toml`:

```toml
[dependencies]
rustica_render = { path = "path/to/rustica_render", features = ["derive"] }
```

The `derive` feature enables the derive macros.

## Integration

Rustica Render Derive integrates with:
- **Render Crate**: Provides derive macros for the render crate
- **Foundation Crate**: Works with geometry types from the foundation crate
