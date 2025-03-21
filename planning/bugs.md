# Rustica Game Engine - Known Bugs and Issues

This document tracks known bugs, issues, and compiler warnings in the Rustica game engine project. Each issue is categorized by severity and includes details on where it occurs and potential remediation steps.

## Critical Issues

### 1. ~~Unresolved import in rustica_math/src/plugin.rs~~ (RESOLVED)

**Description**: ~~The rustica_math crate is trying to import from rustica_core, but there appears to be a dependency or path issue.~~

**Resolution**: The rustica_math module has been removed and its functionality has been migrated to the external cgmath crate, which is now properly re-exported through rustica/src/lib.rs.

### 2. ~~Starfield Example Direct Import Issues~~ (RESOLVED)

**Description**: The starfield example directly imports from rustica_ecs, rustica_scheduler, and cgmath crates, but should use the rustica prelude. This causes compilation failures.

**Error Messages**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `rustica_ecs`
 --> examples\starfield\src\physics.rs:8:5
  |
8 | use rustica_ecs::component::Component;
  |     ^^^^^^^^^^^ use of undeclared crate or module `rustica_ecs`

error[E0432]: unresolved import `cgmath`
 --> examples\starfield\src\physics.rs:7:5
  |
7 | use cgmath::{Vector3 as Vec3};
  |     ^^^^^^ use of undeclared crate or module `cgmath`
```

**Resolution**: Updated the imports in starfield's physics.rs to use the rustica prelude:
- Replaced direct imports with `use rustica::prelude::*;`
- Removed explicit imports of rustica_scheduler::system::SystemFn since it's re-exported in the prelude

## Warnings

### 1. ~~Workspace Resolver Configuration Warning~~ (RESOLVED)

```
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver specify `workspace.resolver = "2"` in the workspace root's manifest
```

**Description**: ~~The workspace is defaulting to resolver version 1, but some crates are using edition 2021 which suggests resolver version 2 should be used.~~

**Resolution**: Verified resolver = "2" is already set in root Cargo.toml.

### 2. ~~Unused Imports in rustica_math/src/lib.rs~~ (RESOLVED)

**Description**: ~~Multiple unused imports in the rustica_math library.~~

**Resolution**: The rustica_math module has been migrated to the cgmath crate.

### 3. ~~Unused Import in rustica_math/src/lib.rs~~ (RESOLVED)

**Description**: ~~The std::fmt module is imported but never used.~~

**Resolution**: The rustica_math module has been migrated to the cgmath crate.

### 4. Unused Import in rustica_core/src/app.rs

```rust
warning: unused import: `crate::CoreError`
  --> crates\rustica_core\src\app.rs:24:5
   |
24 | use crate::CoreError;
   |     ^^^^^^^^^^^^^^^^
```

**Description**: The CoreError type is imported but never used in the app.rs file.

**Remediation**: Remove the unused import or use it in the implementation.

### 5. Unused Variable in rustica_core/src/lib.rs

```rust
warning: unused variable: `app`
  --> crates\rustica_core\src\lib.rs:69:13
   |
69 |         let app = App::new();
   |             ^^^ help: if this is intentional, prefix it with an underscore: `_app`
```

**Description**: There's an unused variable in a test or example in the rustica_core crate.

**Remediation**: Either use the variable, remove it, or prefix with an underscore to indicate intentional non-use.

### 6. Multiple Compiler Warnings Across Crates

```
warning: unused import: `std::any::TypeId`
  --> crates\rustica_ecs\src\component.rs:18:5

warning: methods `len` `is_empty` and `entity_ids` are never used
   --> crates\rustica_ecs\src\component.rs:214:19

warning: unused variable: `query_result`
   --> crates\rustica_ecs\src\query.rs:125:13

warning: unused variable: `app`
  --> crates\rustica_scheduler\src\plugin.rs:14:21

warning: unused import: `rustica_ecs::component::Component`
 --> crates\rustica_render\src\renderer.rs:7:5
```

**Description**: Multiple compiler warnings for unused imports, unused variables, and dead code across various crates.

**Remediation**: Clean up the codebase by addressing each warning:
- Remove unused imports
- Utilize or prefix unused variables with underscore
- Address dead code warnings by either using the methods or marking them with #[allow(dead_code)]

## Build Status Summary

- ~~Unable to compile `rustica_math` due to unresolved import~~ (RESOLVED: rustica_math removed and migrated to cgmath)
- ~~Need to address the workspace resolver configuration~~ (RESOLVED: resolver = "2" set in root Cargo.toml)
- Unable to compile starfield example due to direct import issues
- Multiple warnings across different crates

Last updated: 3/20/2025
