# Rustica Rendering System Guide

## Overview

The `rustica_render` crate provides the rendering system for the Rustica game engine. It handles all aspects of rendering graphics, including managing graphical resources, rendering components, and coordinating with the ECS system to efficiently render game objects.

## Core Components

### Renderer

The core of the rendering system is the `Renderer` struct, which is responsible for:

- Managing rendering resources (textures, meshes, materials)
- Handling the rendering pipeline
- Coordinating with the ECS system to render entities
- Managing the active camera

### RenderComponent

The `RenderComponent` is attached to entities that need to be rendered. It contains:

- References to mesh and material resources
- Visibility settings
- Other render-specific properties

### RenderResource

The `RenderResource` enum represents different types of resources used in rendering:

- Textures
- Meshes
- Materials
- Shaders (to be implemented)

### Plugin Integration

The `RenderPlugin` provides integration with the Rustica engine's plugin system, registering the necessary resources and systems for rendering.

## Usage Examples

### Setting Up the Renderer

```rust
// In your application setup:
App::new()
    .add_plugin(RenderPlugin::default())
    // ...more setup
    .run();
```

### Creating a Renderable Entity

```rust
// Create a cube mesh resource
let cube_mesh_id = renderer.add_resource(RenderResource::Mesh {
    vertices: create_cube_vertices(),
    indices: create_cube_indices(),
});

// Create a material resource
let material_id = renderer.add_resource(RenderResource::Material {
    properties: HashMap::from([
        ("diffuse_color".to_string(), 1.0),
        ("specular_intensity".to_string(), 0.5),
    ]),
});

// Create an entity with a render component
world.spawn()
    .insert(Transform::default())
    .insert(RenderComponent {
        mesh_id: cube_mesh_id,
        material_id: material_id,
        visible: true,
    });
```

### Rendering System

```rust
// A simplified rendering system
fn render_system(
    renderer: Res<Renderer>,
    camera_query: Query<(&Camera, &Transform)>,
    renderable_query: Query<(&RenderComponent, &Transform)>,
) {
    // Get camera
    let (camera, camera_transform) = camera_query.single();
    
    // For each renderable entity
    for (render_component, transform) in renderable_query.iter() {
        if render_component.visible {
            // Render the entity using the renderer
            // (Implementation details would depend on the graphics API)
        }
    }
}
```

## Best Practices

1. Keep rendering logic separate from game logic
2. Use the ECS system to manage render components and their lifecycle
3. Consider batching similar rendering operations for better performance
4. Implement culling techniques to avoid rendering objects outside the camera view
5. Use appropriate level of detail (LOD) techniques for complex scenes
