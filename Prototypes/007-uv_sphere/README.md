# UV Sphere Prototype

This prototype demonstrates the implementation of a UV Sphere in the Rustica engine. It creates a 3D sphere with proper texture coordinates based on longitude and latitude.

## Features

- UV Sphere generation with configurable radius and resolution
- Proper texture coordinate mapping for spherical textures
- Normal calculation for lighting
- Smooth color gradient based on position
- Rotation animation

## Implementation Details

The UV Sphere is generated using spherical coordinates:
- Longitude segments around the equator (0 to 2π)
- Latitude segments from pole to pole (0 to π)

Each vertex includes:
- Position in 3D space
- Normal vector (pointing outward from center)
- Texture coordinates (u,v) mapped from longitude/latitude
- Color (gradient based on position)

## Migration to Graphics Crate

The UV Sphere implementation has been migrated to the graphics crate. You can find it at:

```
crates/graphics/src/primitives/sphere.rs
```

The implementation provides three functions:

1. `create_uv_sphere` - Create a UV Sphere with custom parameters
2. `create_solid_sphere` - Create a UV Sphere with a solid color
3. `create_default_sphere` - Create a UV Sphere with default parameters

### Usage in Graphics Crate

To use the UV Sphere in your application:

```rust
use rustica_graphics::primitives::sphere::{create_uv_sphere, create_solid_sphere, create_default_sphere};

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

## Prototype Usage

This prototype now serves as a minimal demonstration of the UV Sphere functionality. The actual implementation has been moved to the graphics crate.

To run this prototype:

```
cd Prototypes/007-UVSphere
cargo run
```

The prototype demonstrates a rotating UV Sphere with a color gradient based on position.
