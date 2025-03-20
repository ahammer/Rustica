# Rustica Game Engine Backlog

This document contains the prioritized backlog for the Rustica game engine project. Items are organized by priority level and category, with the focus on building an incremental path to a working starfield demo and eventually a high-performance Rust game engine with ray tracing and Vulkan capabilities.

## Priority Levels

- **P0**: Critical - Must be addressed immediately
- **P1**: High - Required for MVP functionality
- **P2**: Medium - Important features for enhanced functionality
- **P3**: Low - Nice-to-have features for future consideration

## Backlog Items

### Critical Bug Fixes (P0)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| BUG-001 | ~~Fix rustica_math module imports~~ | ~~Fix unresolved import `rustica_core` in rustica_math/src/plugin.rs~~ | Low | None |
| BUG-002 | ~~Update workspace resolver~~ | ~~Configure workspace to use resolver "2" to match edition 2021~~ | Low | None |
| BUG-003 | Clean up unused imports | Remove unused imports in rustica_core | Low | None |

### MVP Starfield Implementation (P1)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| MVP-001 | Basic Renderer interface | Create a minimal rendering interface for displaying points/stars | Medium | BUG-002 |
| MVP-002 | Window creation | Implement basic window creation and management | Medium | MVP-001 |
| MVP-003 | Star rendering | Implement basic star rendering with points of varying sizes | Medium | MVP-001, MVP-002 |
| MVP-004 | Position/Velocity system | Implement system for updating positions based on velocities | Medium | None |
| MVP-005 | Enable app.run() loop | Implement the main application loop in the starfield example | Medium | MVP-002, MVP-004 |
| MVP-006 | Basic input handling | Add simple keyboard input to control camera or star movement | Low | MVP-002, MVP-005 |
| MVP-007 | Star wrapping | Implement logic to wrap stars back into view when they move off-screen | Low | MVP-004 |

### Basic Rendering Capabilities (P1)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| RENDER-001 | 2D rendering primitives | Implement basic 2D primitives (points, lines, rectangles) | Medium | MVP-001 |
| RENDER-002 | Z-ordering | Implement proper depth ordering for stars | Medium | MVP-003 |
| RENDER-003 | Star brightness | Implement brightness variation based on distance | Low | MVP-003 |
| RENDER-004 | Simple camera | Implement a basic camera system for the starfield | Medium | MVP-002, MVP-005 |
| RENDER-005 | Frame timing | Add proper frame timing and delta time calculations | Low | MVP-005 |

### Core ECS Enhancements (P1)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| ECS-001 | Complete component storage | Finalize component storage implementation | Medium | None |
| ECS-002 | Query optimization | Optimize entity and component queries | Medium | ECS-001 |
| ECS-003 | World implementation | Complete world implementation with proper entity management | High | ECS-001 |
| ECS-004 | System scheduling | Implement system scheduling with dependencies | High | ECS-003 |
| ECS-005 | Resource management | Add resource management to the ECS world | Medium | ECS-003 |

### Starfield Enhancements (P2)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| STAR-001 | Star twinkle effects | Add visual effects for star twinkling | Low | RENDER-003 |
| STAR-002 | Procedural star generation | Generate stars procedurally based on parameters | Medium | MVP-003 |
| STAR-003 | Star colors | Add color variations to stars | Low | RENDER-001 |
| STAR-004 | Parallax scrolling | Implement parallax effect based on star distance | Medium | RENDER-002, MVP-004 |
| STAR-005 | Star types/classification | Implement various star types with different visual properties | Medium | STAR-003 |
| STAR-006 | Performance optimization | Optimize star rendering for large numbers of stars | High | RENDER-001, ECS-002 |

### Cube/Environment Rendering (P2)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| ENV-001 | Basic 3D primitives | Implement 3D primitives (cubes, spheres) | High | RENDER-001 |
| ENV-002 | Cube rendering | Implement a textured cube renderer | Medium | ENV-001 |
| ENV-003 | Basic camera controls | Add camera controls for exploring 3D space | Medium | RENDER-004, ENV-001 |
| ENV-004 | Simple skybox | Implement a basic skybox for background rendering | Medium | ENV-001 |
| ENV-005 | Basic lighting | Add simple lighting model for 3D objects | High | ENV-001 |

### Advanced Rendering Features (P3)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| ADV-001 | Vulkan integration | Implement Vulkan backend for rendering | Very High | ENV-001, ENV-005 |
| ADV-002 | Shader system | Create a flexible shader system | High | ADV-001 |
| ADV-003 | Basic ray tracing | Implement simple ray tracing capabilities | Very High | ADV-001, ADV-002 |
| ADV-004 | Material system | Implement a flexible material system | High | ADV-002 |
| ADV-005 | Post-processing effects | Add support for post-processing effects | High | ADV-002 |
| ADV-006 | Advanced ray tracing | Implement more advanced ray tracing features (reflections, GI, etc.) | Very High | ADV-003 |

### Tooling & Developer Experience (P3)

| ID | Title | Description | Complexity | Dependencies |
|----|-------|-------------|------------|--------------|
| TOOL-001 | Debug visualization | Add tools for visualizing ECS entities and components | Medium | ECS-003 |
| TOOL-002 | Performance profiling | Implement profiling tools for system performance | Medium | ECS-004 |
| TOOL-003 | Hot reloading | Add support for hot reloading assets and possibly code | High | ADV-002 |
| TOOL-004 | Documentation generation | Improve documentation generation and examples | Medium | None |

## Guiding Principles

1. **Incremental Development**: Focus on getting the basic starfield demo working first before adding more advanced features.

2. **Test Early, Test Often**: Write tests alongside implementation to ensure quality.

3. **Prioritize Working Software**: Ensure each stage results in working software rather than implementing many half-finished features.

4. **Address Technical Debt Promptly**: Don't let technical debt accumulate; fix bugs and refactor as needed.

## Sprint Planning Guidelines

1. Group related tasks that can be completed together.
2. Ensure each sprint has a clear, achievable goal.
3. Include a mix of feature implementation and technical debt reduction.
4. Re-evaluate priorities at the end of each sprint.

## Progress Tracking

The [sprints](./sprints) directory contains records of sprint planning and completion status.
