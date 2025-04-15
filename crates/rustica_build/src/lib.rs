use anyhow::{Context, Result};
use wgsl_to_wgpu::{ValidationOptions, WriteOptions};
use std::path::{Path, PathBuf};
use std::fs;
use std::env;

/// Generates Rust bindings for all WGSL shader files found in the `assets/shaders`
pub fn generate_shader_bindings() -> Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .context("Failed to get CARGO_MANIFEST_DIR environment variable. This function should be called from a build script.")?;
    let out_dir = env::var("OUT_DIR")
        .context("Failed to get OUT_DIR environment variable. This function should be called from a build script.")?;

    let manifest_path = PathBuf::from(manifest_dir);
    let out_path = PathBuf::from(out_dir);
    let shaders_dir = manifest_path.join("assets").join("shaders");

    if !shaders_dir.exists() {
        // If the directory doesn't exist, it's not an error, just means no shaders to process.
        println!("cargo:warning=Shaders directory not found at {}, skipping binding generation.", shaders_dir.display());
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", shaders_dir.display()); // Rerun if the whole directory changes

    for entry in fs::read_dir(&shaders_dir)
        .with_context(|| format!("Failed to read shaders directory: {}", shaders_dir.display()))?
    {
        let entry = entry?;
        let wgsl_path = entry.path();

        if wgsl_path.is_file() && wgsl_path.extension().map_or(false, |ext| ext == "wgsl") {
            let file_stem = wgsl_path.file_stem()
                .context("Failed to get file stem for WGSL file")?
                .to_string_lossy();

            let output_filename = format!("{}.rs", file_stem);
            let output_path = out_path.join(&output_filename);

            println!("cargo:rerun-if-changed={}", wgsl_path.display()); // Rerun if individual shader changes

            let wgsl_source = fs::read_to_string(&wgsl_path)
                .with_context(|| format!("Failed to read WGSL shader file: {}", wgsl_path.display()))?;

            let options = WriteOptions::default(); // Consider making options configurable

            let rust_bindings_source = wgsl_to_wgpu::create_shader_module(&wgsl_source, wgsl_path.to_string_lossy().as_ref(), options)
                .with_context(|| format!("Failed to generate bindings for {}", wgsl_path.display()))?;

            fs::write(&output_path, rust_bindings_source)
                .with_context(|| format!("Failed to write generated bindings to {}", output_path.display()))?;

            println!("Generated bindings for {} at {}", wgsl_path.display(), output_path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::env;

    // Test the modified function which relies on environment variables
    // and scans a predefined directory structure.
    #[test]
    fn test_generate_bindings_from_dir() -> Result<()> {
        let base_dir = tempdir()?;
        let manifest_dir = base_dir.path();
        let out_dir = base_dir.path().join("target").join("debug").join("build").join("test_crate-12345"); // Simulate OUT_DIR structure
        let assets_shaders_dir = manifest_dir.join("assets").join("shaders");

        fs::create_dir_all(&assets_shaders_dir)?;
        fs::create_dir_all(&out_dir)?;

        // Set environment variables for the test
        env::set_var("CARGO_MANIFEST_DIR", manifest_dir.to_str().unwrap());
        env::set_var("OUT_DIR", out_dir.to_str().unwrap());

        let wgsl_path1 = assets_shaders_dir.join("test_shader1.wgsl");
        let wgsl_content1 = r#"
            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                let x = f32(i32(in_vertex_index) - 1);
                let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                return vec4<f32>(x, y, 0.0, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(0.0, 1.0, 0.0, 1.0); // Green
            }
        "#;
        fs::write(&wgsl_path1, wgsl_content1)?;

        let wgsl_path2 = assets_shaders_dir.join("test_shader2.wgsl");
        let wgsl_content2 = r#"
            struct Uniforms {
               color: vec4<f32>,
            };
            @group(0) @binding(0) var<uniform> uniforms: Uniforms;

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return uniforms.color;
            }
        "#;
        fs::write(&wgsl_path2, wgsl_content2)?;

        // Create a non-wgsl file to ensure it's ignored
        let non_wgsl_path = assets_shaders_dir.join("notes.txt");
        fs::write(&non_wgsl_path, "This should be ignored")?;


        generate_shader_bindings()?;

        let output_path1 = out_dir.join("test_shader1.rs");
        let output_path2 = out_dir.join("test_shader2.rs");
        let ignored_output_path = out_dir.join("notes.rs"); // Should not exist

        assert!(output_path1.exists(), "Output file for test_shader1.wgsl was not created");
        assert!(output_path2.exists(), "Output file for test_shader2.wgsl was not created");
        assert!(!ignored_output_path.exists(), "Output file for notes.txt should not have been created");

        let generated_code1 = fs::read_to_string(&output_path1)?;
        assert!(generated_code1.contains("vs_main"));
        assert!(generated_code1.contains("fs_main"));

        let generated_code2 = fs::read_to_string(&output_path2)?;
        assert!(generated_code2.contains("struct Uniforms"));
        assert!(generated_code2.contains("fs_main"));

        // Clean up environment variables
        env::remove_var("CARGO_MANIFEST_DIR");
        env::remove_var("OUT_DIR");

        Ok(())
    }

    #[test]
    fn test_generate_bindings_no_shader_dir() -> Result<()> {
        let base_dir = tempdir()?;
        let manifest_dir = base_dir.path();
        // DO NOT create assets/shaders dir
        let out_dir = base_dir.path().join("target").join("debug").join("build").join("test_crate_no_shader-12345");
        fs::create_dir_all(&out_dir)?;


        // Set environment variables for the test
        env::set_var("CARGO_MANIFEST_DIR", manifest_dir.to_str().unwrap());
        env::set_var("OUT_DIR", out_dir.to_str().unwrap());

        // Call the function - it should succeed even if the dir doesn't exist
        generate_shader_bindings()?;

        // Assert that no files were created in out_dir
        assert!(fs::read_dir(&out_dir)?.next().is_none(), "No files should be created in OUT_DIR when assets/shaders doesn't exist");

        // Clean up environment variables
        env::remove_var("CARGO_MANIFEST_DIR");
        env::remove_var("OUT_DIR");

        Ok(())
    }
}
