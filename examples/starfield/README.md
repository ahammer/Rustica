# Starfield Example

This example demonstrates a simple "hello world" application using the Rustica game engine. It creates a starfield of 5 stars with positions, velocities, and visual properties.

## Structure

- **Components**: Defines `Position`, `Velocity`, and `Star` components
- **Plugin**: Implements a `StarfieldPlugin` that spawns star entities
- **Systems**: Contains commented examples of how systems would be implemented

## Running the Example

To run this example:

```bash
# From the workspace root
cargo run --bin starfield
```

## What This Example Demonstrates

1. **Engine Structure**: Shows the architecture of Rustica with plugins, components, and systems
2. **ECS Pattern**: Demonstrates the entity-component-system pattern
3. **Plugin System**: Shows how to extend the engine with custom functionality

This is a minimal example that doesn't produce actual visual output, but demonstrates the API structure and usage patterns of the Rustica engine.

## Next Steps

In a full implementation:

1. The engine would include a rendering system to display the stars
2. Systems would update the positions based on velocities each frame
3. Stars would be rendered with different sizes and brightnesses
4. Stars would wrap around when they reach the edge of the screen
