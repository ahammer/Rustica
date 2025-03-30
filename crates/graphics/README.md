# Rustica Graphics

The Graphics crate provides high-level graphics primitives and utilities for the Rustica engine.

## Overview

Rustica Graphics builds upon the Foundation and Render crates to provide ready-to-use graphics primitives and utilities. It offers a higher-level API for common graphics operations, making it easier to create 3D scenes.

## Features

- **Camera System**: Perspective and orthographic cameras with view and projection matrices
- **Mesh Primitives**: Pre-built mesh types for common 3D objects
- **UV Sphere**: Configurable UV sphere implementation with texture coordinate mapping
- **PBR Shaders**: Physically-based rendering shaders for realistic materials

## Usage

### Creating and Using a Camera

```rust
use rustica_graphics::primitives::camera::{Camera, CameraMatrices};
use glam::{Vec3, Mat4};

// Create a perspective camera
let mut camera = Camera::new_perspective(
    45.0,           // field of view in degrees
    800.0 / 600.0,  // aspect ratio
    0.1,            // near plane
    100.0           // far plane
);

// Position and orient the camera
camera.set_position(Vec3::new(0.0, 0.0, 5.0));
camera.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

// Get camera matrices for rendering
let matrices: CameraMatrices = camera.matrices();
let view_matrix: Mat4 = matrices.view;
let projection_matrix: Mat4 = matrices.projection;
```

### Creating a UV Sphere

```rust
use rustica_graphics::primitives::sphere::{create_uv_sphere, create_solid_sphere, create_default_sphere};
use rustica_foundation::prelude::*;

// Create a default sphere (radius 1.0, 32x16 segments, gradient color)
let default_sphere = create_default_sphere();

// Create a custom sphere
let custom_sphere = create_uv_sphere(
    2.0,            // radius
    64,             // longitude segments
    32,             // latitude segments
    Some([1.0, 0.0, 0.0])  // red color (optional)
);

// Create a solid color sphere
let blue_sphere = create_solid_sphere(
    1.5,            // radius
    48,             // longitude segments
    24,             // latitude segments
    [0.0, 0.0, 1.0] // blue color
);
```

### Using Meshes

```rust
use rustica_graphics::primitives::mesh::MeshBuilder;
use rustica_foundation::prelude::*;

// Create a mesh using the builder pattern
let mesh = MeshBuilder::new()
    .with_vertex(StandardVertex::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0]))
    .with_vertex(StandardVertex::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0], [1.0, 1.0, 1.0]))
    .with_vertex(StandardVertex::new([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0], [1.0, 1.0, 1.0]))
    .with_face(Face::new(0, 1, 2))
    .build();
```

## Integration

Rustica Graphics integrates with:
- **Foundation Crate**: Uses geometry types for primitives
- **Render Crate**: Provides primitives that can be rendered
