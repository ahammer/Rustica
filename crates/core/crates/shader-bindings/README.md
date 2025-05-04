# Rustica Shader Bindings

This crate provides Rust bindings for WGSL shaders used in the Rustica engine. It uses `wgsl_bindgen` to automatically generate type-safe Rust representations of shader structures and bind groups.

## Features

- Type-safe structs that match WGSL shader definitions
- `bytemuck::Pod` implementations for all shader types
- Vertex buffer layout definitions
- Bind group layout descriptors
- Support for efficient memory layouts with proper padding

## Usage

### Basic Example

```rust
use rustica_shader_bindings::pbr::{
    CameraUniform,
    ModelUniform,
    MaterialUniformInit,
    VertexInput,
    WgpuBindGroup0,
    WgpuBindGroup1,
    WgpuBindGroup2
};
use glam::{Mat3A, Mat4, Vec3A, Vec4};

// Create shader uniforms
let camera = CameraUniform {
    view_proj: Mat4::IDENTITY,
    position: Vec3A::ZERO,
};

let model = ModelUniform {
    model: Mat4::IDENTITY,
    normal_transform: Mat3A::IDENTITY,
};

// Use the Init struct for easier initialization
let material_init = MaterialUniformInit {
    base_color_factor: Vec4::ONE,
    metallic_factor: 0.5,
    roughness_factor: 0.5,
};
let material = material_init.into(); // Converts to MaterialUniform with padding

// Create vertex data
let vertex = VertexInput {
    position: Vec3A::new(0.0, 0.0, 0.0),
    normal: Vec3A::new(0.0, 1.0, 0.0),
    uv: [0.0, 0.0],
};

// Create buffers (using a GPU context)
let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Camera Uniform Buffer"),
    contents: bytemuck::bytes_of(&camera),
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
});

// Get bind group layouts
let camera_bg_layout = WgpuBindGroup0::get_bind_group_layout(&device);
```

### Adding New Shaders

To add a new shader:

1. Create a WGSL shader file in the `shaders/` directory
2. Update the `build.rs` script to include the new shader
3. Re-export the generated module from `lib.rs`

## Implementation Details

- Uses `wgsl_bindgen` to parse WGSL shaders at build time
- Maps WGSL types to `glam` types for efficient math operations
- Handles memory layout and padding automatically
- Generates bytemuck trait implementations for zero-copy buffer operations

## Notes

- All generated code includes proper layout assertions
- Vertex buffer layouts are automatically created based on shader attributes
- Bind group layouts match the shader's bind group declarations
