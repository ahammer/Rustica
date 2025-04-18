# Utah Teapot Prototype

This prototype demonstrates rendering the classic Utah teapot using Bezier patches in the Rustica engine.

It's jank as hell, the LLM didn't get it right, but the patches seem to work. I'll probably refine this POC to another more predictable NURBS/Bezier surfaces,
## Features

- Implementation of Bezier curves and patches in the Foundation crate
- Utah teapot model defined using 32 bicubic Bezier patches
- Tessellation of Bezier patches into triangle meshes
- Phong lighting model in the shader for realistic rendering
- Animated rotating teapot with moving light source

## Technical Details

The Utah teapot is a classic 3D model in computer graphics, originally created by Martin Newell in 1975. This implementation:

1. Defines the teapot using 32 bicubic Bezier patches
2. Tessellates each patch into a triangle mesh
3. Combines all patches into a single mesh
4. Renders the mesh with a shader that implements the Phong lighting model

The Bezier implementation in the Foundation crate supports:
- Generic Bezier curves with any control point type
- 2D and 3D Bezier curves
- Bezier patches (surfaces) for 3D modeling
- Tessellation of curves and patches

## Running the Prototype

```bash
cd Prototypes/008-Teapot
cargo run
```
