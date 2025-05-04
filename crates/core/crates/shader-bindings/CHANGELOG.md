# Changelog

All notable changes to the `rustica-shader-bindings` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-05-03

### Added
- Initial implementation of PBR shader bindings
- Generated type-safe Rust bindings for WGSL shader structures
- Automatic `bytemuck::Pod` and `bytemuck::Zeroable` implementations for memory layout compatibility
- Layout assertions to verify host-device memory layout matches
- Clean public API through the `pbr_shader` module
- Support for binding uniform buffers through WGPU bind groups
