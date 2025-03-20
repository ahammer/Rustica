# Rustica Game Engine - Known Bugs and Issues

This document tracks known bugs, issues, and compiler warnings in the Rustica game engine project. Each issue is categorized by severity and includes details on where it occurs and potential remediation steps.

## Critical Issues

### 1. Unresolved import in rustica_math/src/plugin.rs

```rust
error[E0432]: unresolved import `rustica_core`
--> crates\rustica_math\src\plugin.rs:3:5
   |
3  | use rustica_core::{App Plugin};
   |     ^^^^^^^^^^^^ use of undeclared crate or module `rustica_core`
```

**Description**: The rustica_math crate is trying to import from rustica_core, but there appears to be a dependency or path issue.

**Remediation**: 
- Add rustica_core as a dependency in rustica_math's Cargo.toml
- Fix the import syntax (missing comma between App and Plugin)
- Ensure proper re-exports are configured

## Warnings

### 1. Workspace Resolver Configuration Warning

```
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver specify `workspace.resolver = "2"` in the workspace root's manifest
```

**Description**: The workspace is defaulting to resolver version 1, but some crates are using edition 2021 which suggests resolver version 2 should be used.

**Remediation**: Add the following to the workspace section in the root Cargo.toml:
```toml
[workspace]
resolver = "2"
```

### 2. Unused Imports in rustica_math/src/lib.rs

```rust
warning: unused imports: `AddAssign` `Add` `DivAssign` `Div` `MulAssign` `Mul` `Neg` `SubAssign` and `Sub`
  --> crates\rustica_math\src\lib.rs:27:16
   |
27 | use std::ops::{Add Sub Mul Div AddAssign SubAssign MulAssign DivAssign Neg};
   |                ^^^ ^^^ ^^^ ^^^ ^^^^^^^^^ ^^^^^^^^^ ^^^^^^^^^ ^^^^^^^^^ ^^^
```

**Description**: Multiple unused imports in the rustica_math library.

**Remediation**: Remove unused imports or use them in the implementation. Also fix the syntax (missing commas between imports).

### 3. Unused Import in rustica_math/src/lib.rs

```rust
warning: unused import: `std::fmt`
  --> crates\rustica_math\src\lib.rs:28:5
   |
28 | use std::fmt;
   |     ^^^^^^^^
```

**Description**: The std::fmt module is imported but never used.

**Remediation**: Remove the unused import or use it in the implementation.

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

## Build Status Summary

- Unable to compile `rustica_math` due to unresolved import
- Multiple warnings across different crates
- Need to address the workspace resolver configuration

Last updated: 3/19/2025
