# ECS Prototype: Conway's Game of Life

This prototype demonstrates a simple Entity Component System (ECS) architecture implemented from scratch. The primary goal is to showcase a clean, elegant syntax for working with ECS while implementing Conway's Game of Life as a practical example.

## ECS Implementation

The ECS contains these core components:

1. **Entity**: A simple ID (unsigned integer) that serves as a unique identifier.
2. **Component**: Pure data structures that can be attached to entities.
3. **System**: Logic that operates on entities with specific components.
4. **World**: Container that manages entities, components, and systems.

## Key Features

- Fluent entity creation API with builder pattern: `world.create_entity().with(Component1).with(Component2).build()`
- Type-safe component storage using Rust's type system
- Elegant query syntax: `world.query::<(Position, CellState)>()` returns entities with both components
- Systems that can operate on queried components

## Conway's Game of Life

This implementation represents Conway's Game of Life where:

- Each cell is an entity
- Each entity has a Position component (x, y) and a CellState component (alive/dead)
- The LifeSystem applies the standard Conway rules:
  1. Any live cell with fewer than two live neighbors dies (underpopulation)
  2. Any live cell with two or three live neighbors lives on
  3. Any live cell with more than three live neighbors dies (overpopulation)
  4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction)
- The RenderSystem displays the grid in the terminal

## Usage

Run the example with:

```bash
cargo run
```

The simulation starts with a glider pattern and will continue running until you press Ctrl+C.

## Design Considerations

- The ECS is intentionally simple and does not handle all edge cases that a production ECS would need
- Component storage uses a simple HashMap for clarity, not optimized for cache-locality
- No parallelism is implemented in this prototype

## Future Improvements

- Sparse storage for components to improve performance
- Parallel system execution
- Resource management
- Events/messaging
- System ordering and dependencies
