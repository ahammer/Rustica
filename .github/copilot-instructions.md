### Rustica Engineering Charter (TL;DR)
1. **Bold fixes first** – prefer clear, breaking improvements over timid patches.
2. **Definition of Done**  
   - all `cargo test` suites pass  
   - no `clippy --all-targets -- -D warnings` violations  
   - public items have rust-doc examples  
   - CHANGELOG entry created if API surface changes
3. **Testing discipline** – every public fn gets at least one positive & one negative test.
4. **Dependency discipline** - Also use workspace dependencies for versioning
5. **Public Api** - Only Glam, Rustica are allowed public API. WGPU is implementation detail.
6. **Deprecations** - Delete old code, don't deprecate it. This includes comments, tests, and examples. DO NOT COMMENT, DELETE