use anyhow::{Context, Result};
use std::path::Path;

/// Generates Rust bindings for a WGSL shader file using `wgsl_to_wgpu`.
///
/// # Arguments
///
/// * `wgsl_path` - The path to the input WGSL shader file.
/// * `output_path` - The path where the generated Rust file should be saved.
/// * `shader_name` - A name for the shader module (e.g., "StandardShader"). This helps
///   in organizing the generated code, especially struct names.
///
/// # Returns
///
/// `Ok(())` if generation is successful, otherwise an `Err` containing the error details.
pub fn generate_shader_bindings(
    wgsl_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    shader_name: &str,
) -> Result<()> {
    let wgsl_path = wgsl_path.as_ref();
    let output_path = output_path.as_ref();

    println!("cargo:rerun-if-changed={}", wgsl_path.display());

    let wgsl_source = std::fs::read_to_string(wgsl_path)
        .with_context(|| format!("Failed to read WGSL shader file: {}", wgsl_path.display()))?;

    let options = wgsl_to_wgpu::WriteOptions {
        derive_bytemuck: true, // Essential for buffer usage
        derive_encase: true,   // Useful for uniform buffers
        matrix_vector_types: wgsl_to_wgpu::MatrixVectorTypes::Glam, // Use glam types
        derive_serde: false, // Not needed for this use case
    };

    let rust_bindings_source = wgsl_to_wgpu::create_shader_module(&wgsl_source, wgsl_path.to_string_lossy().as_ref(), options)
        .with_context(|| format!("Failed to generate bindings for {}", wgsl_path.display()))?;

    // Ensure the output directory exists
    if let Some(parent_dir) = output_path.parent() {
        std::fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create output directory: {}", parent_dir.display()))?;
    }

    std::fs::write(output_path, rust_bindings_source)
        .with_context(|| format!("Failed to write generated bindings to {}", output_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // Basic test to ensure the function runs without panicking
    // and creates an output file. More comprehensive tests would
    // involve compiling the generated code.
    #[test]
    fn test_generate_basic_shader() -> Result<()> {
        let dir = tempdir()?;
        let wgsl_path = dir.path().join("test.wgsl");
        let output_path = dir.path().join("bindings.rs");

        let wgsl_content = r#"
            struct VertexInput {
                @location(0) position: vec3<f32>,
            };

            @vertex
            fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
                return vec4<f32>(in.position, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        std::fs::write(&wgsl_path, wgsl_content)?;

        generate_shader_bindings(&wgsl_path, &output_path, "TestShader")?;

        assert!(output_path.exists(), "Output file was not created");
        let generated_code = std::fs::read_to_string(&output_path)?;
        assert!(generated_code.contains("struct TestShaderVertexInput")); // Check for namespaced struct
        assert!(generated_code.contains("#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable)]")); // Check for bytemuck derive

        Ok(())
    }
}
