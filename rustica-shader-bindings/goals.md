
## Vision & Scope

`wgsl-bindgen` will be the **canonical source of truth** for GPU-side data definitions inside the Rustica ecosystem.
It parses WGSL at build time and emits strongly-typed Rust modules that guarantee:

* **Host–device memory layout matches** (verified with `bytemuck`/`encase` or equivalent) ([Docs.rs][1])
* **Bind-group layouts & pipeline layouts** are generated once and reused, eliminating hand-rolled boilerplate ([GitHub][2])
* **Compile-time drift detection**: any mismatch between shader edits and host code surfaces as a Rust compilation error rather than a GPU panic ([Docs.rs][3])

The crate is ***not*** a rendering framework, scene graph, or material system — it is focused purely on binding-generation plumbing 


## Deliverables

### 1. `wgsl-bindgen-core`

* A no-std compatible library containing:

  * WGSL token → Rust token translation powered by **`naga-oil`** for AST and layout data ([Docs.rs][1])
  * Convenience builders (`WgslBindgenOptionBuilder`) for configuring generation targets, derive strategies, and custom type maps ([Docs.rs][1])
  * Helpers for emitting **`wgpu::BindGroupLayout`** and **`wgpu::PipelineLayoutDescriptor`** stubs ready for device creation ([Docs.rs][5])

### 2. `rustica-foundation`

* A small, re-exporting crate that ships **opinionated defaults**:

  * Glam/Nalgebra type maps out-of-the-box (`Vec3`, `Mat4`, etc.) ([Docs.rs][1])
  * Feature-gated derives for `bytemuck`, `encase`, and `serde` ([GitHub][2])
  * Common utility traits (`AsBindGroup`, `AsVertexBuffer`, etc.) for ergonomic usage in Rustica render code.

### 3. Tests & CI

| Layer                          | Purpose                                                                                  | Strategy                                                                                                                 |
| ------------------------------ | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| **Unit**                       | Validate token translation & layout maths.                                               | Pure Rust tests run under `cargo test`.                                                                                  |
| **Const-assert**               | Ensure host structs exactly match WGSL sizes/alignments.                                 | `bytemuck::checked::assert_eq_size!` integrated into generated code                       |
| **Integration (headless GPU)** | Compile shaders, instantiate pipelines, draw a dummy pass to guarantee runtime validity. | Re-use wgpu’s headless backend in CI; mirrors the infrastructure improvements wgpu adopted for CTS testing
| **Snapshot**                   | Guard against accidental breaking changes in generated Rust.                             | Commit golden files in `tests/snapshots/`; diff them during CI.                                                          |

### 4. Public API Expectations

The surface API must remain **stable semver-wise** for the following key items:

* `wgsl_bindgen::generate()` – procedural entry; **build-script only**.
* `bindings::<shader_mod>::BindGroups::*` – strongly-typed structs mapping to WGSL bind groups.
* `bindings::<shader_mod>::vertex::Vertex` – host-side vertex buffer structs with `VertexBufferLayout` constants
* Generated `pipeline()` helpers returning `(ShaderModule, PipelineLayout)` for fast pipeline creation.

Breaking changes to those items require a **major** version bump and a migration note.

---

## Stretch Goals / Extensibility

* **Shader composition**: basic `include` graph + dependency hashing to support multi-file WGSL modules

---

## Non-Goals

* **Material/scene abstraction** – left to higher-level Rustica crates or game code.
* **Author-time shader IDE** – integration hooks are planned but an actual IDE/VS Code plugin is out of scope ([Reddit][9]).
* **Support for SPIR-V/GLSL ingestion** – WGSL is the canonical shading language per wgpu default features; other translations belong in separate tooling ([wgpu.rs][10]).

