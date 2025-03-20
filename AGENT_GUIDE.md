# Rustica Game Engine - Agent Guide

This document serves as the primary entry point for AI agents working with the Rustica codebase. It provides navigation, rules, and constraints to help maintain code quality and consistency.

## Repository Structure

```
rustica/ (workspace)
├── Cargo.toml                   // Workspace manifest
├── AGENT_GUIDE.md               // This file - agent entry point
├── crates/
│   ├── rustica/                 // Main library crate (re-exports)
│   ├── rustica_core/            // Minimal orchestration layer
│   ├── rustica_ecs/             // ECS implementation
│   ├── rustica_event/           // Event system
│   ├── rustica_render/          // Rendering capabilities
│   ├── rustica_scheduler/       // System scheduling
│   └── rustica_math/            // Math utilities
├── examples/
│   └── starfield/               // Hello World starfield example
└── docs/
    ├── architecture.md          // Architecture overview
    ├── api_conventions.md       // API design rules
    ├── testing_standards.md     // Testing requirements
    └── implementation_rules.md  // Implementation guidelines
```

## Critical Rules and Constraints

1. **Core API Minimalism**: The core module should be minimal, with all functionality moved to subsystems.
2. **Clean Dependency Graph**: Subsystems should be as independent as possible, with clearly defined interfaces.
3. **Complete Documentation**: All public items must be documented with examples.
4. **Test Everything**: All functionality must be testable and include tests.

## API Design Principles

1. **Public vs. Private**: Carefully limit what is exposed through the public API.
2. **Error Handling**: Functions that can fail should return `Result<T, Error>`.
3. **Naming Conventions**: 
   - Types/traits: `PascalCase`
   - Functions/methods: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`
4. **Plugin Consistency**: Plugins must implement the `Plugin` trait and follow a consistent registration pattern.

## Testing Expectations

1. **Unit Tests**: Each module must have unit tests for all public functions.
2. **Integration Tests**: Cross-module functionality must have integration tests.
3. **Headless Testing**: All tests must be able to run without a window/display.

## File Header Standards

Every source file must include a standardized header with:
- Module description
- Links to relevant documentation
- Critical rules for the module
- Example usage

The header follows this format:

```rust
//! # ModuleName: Brief Description
//! 
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#ModuleName
//! - API_RULES: /docs/api_conventions.md#ModuleSpecificRules
//! - TEST_RULES: /docs/testing_standards.md#ModuleTestingPatterns
//!
//! ## Critical Rules
//! 
//! 1. [Rule specific to this module]
//! 2. [Another rule]
//! 
//! ## Usage Example
//! 
//! ```rust
//! // Example showing the intended usage pattern
//! ```
```

## Decision Making Guidelines

When implementing or modifying code, follow these decision trees:

### Adding New Functionality

1. Determine which subsystem the functionality belongs to.
2. Check if existing interfaces can be extended or if new ones are needed.
3. Design the public API first, focusing on usability.
4. Implement with testability as a priority.
5. Add comprehensive documentation.

### Modifying Existing Code

1. Identify all places that could be affected by the change.
2. Check for test coverage of the affected areas.
3. Ensure API compatibility if the change is not breaking.
4. Update documentation to reflect changes.

## Documentation Navigation

- [Architecture Overview](/docs/architecture.md): System design and component relationships
- [API Conventions](/docs/api_conventions.md): Rules for API design
- [Testing Standards](/docs/testing_standards.md): Testing requirements and patterns
- [Implementation Rules](/docs/implementation_rules.md): Guidelines for implementation

## Module-Specific Guidelines

Each module has its own `MODULE_GUIDE.md` with specific conventions and constraints:

- [rustica Module Guide](/crates/rustica/MODULE_GUIDE.md)
- [Core Module Guide](/crates/rustica_core/MODULE_GUIDE.md)
- [ECS Module Guide](/crates/rustica_ecs/MODULE_GUIDE.md)
- [Event Module Guide](/crates/rustica_event/MODULE_GUIDE.md)
- [Render Module Guide](/crates/rustica_render/MODULE_GUIDE.md)
- [Scheduler Module Guide](/crates/rustica_scheduler/MODULE_GUIDE.md)
- [Math Module Guide](/crates/rustica_math/MODULE_GUIDE.md)
