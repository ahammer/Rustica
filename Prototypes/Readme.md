# Rustica Prototypes

The Prototypes directory contains proof-of-concept implementations for various features of the Rustica engine.

## Overview

Prototypes serve as experimental implementations to prove specific concepts before they are integrated into the main engine crates. Each prototype focuses on a single aspect of the system, demonstrating its functionality in isolation.

## Development Workflow

Prototypes follow a specific development workflow:

1. **POC (Proof of Concept)**: Initial implementation with direct dependencies on external libraries
2. **Prove the POC**: Demonstrate that the concept works as expected
3. **Integrate as a Library**: Extract the functionality into the appropriate engine crate
4. **Integrate the Library**: Update the prototype to use the engine crate instead of direct dependencies

## Prototype Rules

- Prototypes have two states: POC and Complete
- In POC state, a prototype may only depend on completed prototypes
- When a prototype is in POC mode, it uses external libraries directly (e.g., winit)
- Once promoted to Complete, it uses only the Rustica crates and no longer has direct external dependencies

## Current Prototypes

1. **001-BasicWindow**: Basic window creation using winit
2. **002-BasicWGPU**: Basic wgpu setup and initialization
3. **003-BasicTriangle**: Rendering a simple triangle with wgpu
4. **004-AnimatedTriangle**: Adding animation to the triangle
5. **005-CustomShader**: Implementing custom shaders
6. **006-BasicMesh**: Basic mesh rendering
7. **007-UVSphere**: UV sphere implementation with texture coordinates

## Development Pattern

Prototypes are developed in layers, like an onion:
1. First, we implemented window management
2. Then, we wrapped that with RenderWindow
3. Next, we added rendering capabilities
4. And so on, building up the engine layer by layer

The latest prototype (currently 007-UVSphere) is typically the one in active development, unless a new prototype is being created.
