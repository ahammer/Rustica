### Rustica Engineering Charter (LLM Guidance)

1.  **API Tiers**: Adhere to the 4-tier structure (`core` -> `canvas` -> `scene` -> `flow`). Higher tiers depend *only* on the immediate lower tier. See `project-structure.md`.
2.  **Public API**: Use only `glam` and `rustica_*` types in public function signatures and struct fields. `wgpu` is an implementation detail.
3.  **Dependencies**: Use `workspace = true` for all dependencies listed in the root `Cargo.toml`. Minimize external crates.
4.  **Code Style**: Pass `clippy --all-targets -- -D warnings`. Format with `rustfmt`.
5.  **Testing**: Every `pub fn` requires one success test (`#[test]`) and one failure/edge-case test (`#[test]`). Visual tests go in `examples/`.
6.  **Documentation**: Add rust-doc examples (`/// # Example`) for all `pub` items. Docs explain *intent* and *invariants* for LLM understanding. Update docs atomically with code.
7.  **Evolution**: Delete unused code/tests/docs. Do not comment out. Prefer breaking changes for clarity over backward compatibility. Update CHANGELOG for public API changes.
8.  **File Structure**: Library crates live in `crates/`. Sub-modules are nested crates (e.g., `crates/core/crates/shader-bindings`). Examples live in `examples/` categorized by tier. See `project-structure.md`.
9. **Versions**: DO NOT CHANGE VERSIONS WITHOUT EXPLICIT CONSENT