# Rustica Examples

The Examples directory contains sample applications that demonstrate how to use the Rustica engine.

## Overview

These examples showcase various features of the Rustica engine and provide a starting point for developers looking to build their own applications. Each example focuses on a specific aspect of the engine and demonstrates best practices for using the Rustica API.

## Current Examples

### 001-Integration

This example demonstrates how to integrate multiple Rustica crates to create a complete application. It shows:

- Window creation and management
- Rendering setup
- Mesh creation and rendering
- Camera setup and control
- Basic application structure

## Running the Examples

To run an example, navigate to its directory and use cargo run:

```bash
cd Examples/001-Integration
cargo run
```

## Creating Your Own Applications

These examples serve as templates for creating your own applications with Rustica. The recommended approach is:

1. Start with the simplest example that's closest to your needs
2. Copy and modify it to create your own application
3. Add additional Rustica features as needed

## Example Structure

Each example follows a similar structure:

- **Cargo.toml**: Dependencies and project configuration
- **src/main.rs**: The main application code
- **src/**: Additional modules specific to the example
- **assets/**: (If applicable) Textures, models, and other assets

## Learning Path

For newcomers to Rustica, we recommend exploring the examples in this order:

1. **001-Integration**: Basic integration of Rustica crates
2. (Future examples will be added here)

This progression introduces concepts gradually, building on previous knowledge.
