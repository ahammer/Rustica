# Rustica Project Structure

## Crates (`crates/`)

-   **`core`**: Low-level WGPU wrappers, pipeline/buffer helpers.
    -   **`core/crates/shader-bindings`**: Type-safe WGSL <-> Rust bridge.
    -   **`core/crates/geometry`**: Vertex formats, mesh data.
-   **`canvas`**: 2D/3D render loop, draw commands (sprites, text), resource management. Depends on `core`.
-   **`scene`**: Declarative scene graph (ECS), `scene!{}` macro. Translates scene state to `canvas` calls. Depends on `canvas`.
-   **`flow`**: Data-driven scene definition (JSON/YAML), node editor schema, runtime loader. Depends on `scene`.

## Examples (`examples/`)

Organized by the highest API tier they use:
-   `examples/core/`
-   `examples/canvas/`
-   `examples/scene/`
-   `examples/flow/`

## Scratchpad (`scratchpad/`)

Temporary, experimental code. Not part of the core engine.

## Workspace (`Cargo.toml`)

Defines all crates and shared dependencies (`workspace.dependencies`).
