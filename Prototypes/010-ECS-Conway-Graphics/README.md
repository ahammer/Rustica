# Prototype 010: ECS Conway's Game of Life with Graphics

This prototype demonstrates the integration of the rustica-ecs and rustica-conway crates with Rustica's rendering capabilities to create a 3D visualization of Conway's Game of Life.

## Overview

Conway's Game of Life is a cellular automaton where cells on a grid live or die according to simple rules based on their neighbors. This prototype:

1. Uses the Entity Component System (ECS) from `rustica-ecs` to manage game logic
2. Draws upon game logic from `rustica-conway` to apply Conway's rules
3. Renders live cells as 3D cubes using shaders and Rustica's rendering system
4. Updates the simulation at fixed intervals

## Key Aspects

- **ECS Integration**: Shows how to use an ECS for game simulation logic separated from rendering
- **Frame Callback**: Uses the `with_frame_callback` method to set up a rendering loop
- **Shader Definition**: Demonstrates using the `ShaderDescriptor` derive macro for shader interfaces
- **Camera Setup**: Creates a perspective camera to view the grid from above
- **Vertex Types**: Uses custom vertex types with attributes for the GPU
- **Cell Transformation**: Positions cells in 3D space based on their grid coordinates
- **Performance**: Only draws cubes for live cells, improving rendering efficiency

## Running the Prototype

Simply run the prototype with:

```
cargo run --package rustica-ecs-conway-graphics
```

Controls:
- The simulation runs automatically at fixed intervals
- Close the window to exit
