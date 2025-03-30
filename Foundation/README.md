# Rustica Foundation

The Foundation crate provides the core geometric primitives and data structures for the Rustica engine.

## Overview

Foundation serves as the base layer for the Rustica engine, providing essential geometric types and traits that are used throughout the engine. It defines the fundamental building blocks for 3D geometry, including vertices, meshes, and faces.

## Features

- **Vertex System**: Defines the `Vertex` trait and provides a standard vertex implementation
- **Mesh Representation**: Data structures for representing 3D meshes
- **Face Handling**: Triangle and face definitions for mesh construction
- **Geometry Traits**: Common interfaces for geometric operations

## Usage

### Working with Vertices

```rust
use rustica_foundation::prelude::*;

// Create a standard vertex with position, normal, and UV coordinates
let vertex = StandardVertex {
    position: [0.0, 0.0, 0.0],
    normal: [0.0, 1.0, 0.0],
    uv: [0.0, 0.0],
    color: [1.0, 1.0, 1.0],
};

// Access vertex attributes
let position = vertex.position;
let normal = vertex.normal;
```

### Creating a Mesh

```rust
use rustica_foundation::prelude::*;

// Create vertices
let vertices = vec![
    StandardVertex::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0]),
    StandardVertex::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0], [1.0, 1.0, 1.0]),
    StandardVertex::new([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0], [1.0, 1.0, 1.0]),
];

// Create faces (triangles)
let faces = vec![
    Face::new(0, 1, 2),
];

// Create a mesh
let mesh = Mesh::new(vertices, faces);
```

## Integration

Foundation is used by:
- **Render Crate**: Uses the geometry types for rendering
- **Graphics Crate**: Builds upon the geometry primitives to create higher-level graphics objects
