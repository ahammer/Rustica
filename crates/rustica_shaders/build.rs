use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Define paths relative to the crate root
    let wgsl_path = PathBuf::from("src/shaders/standard_shader.wgsl");

    // Get the output directory from Cargo
    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .context("Failed to get OUT_DIR environment variable")?);
    let output_path = out_dir.join("standard_shader_bindings.rs");

    // Generate the bindings
    rustica_build::generate_shader_bindings(
        &wgsl_path,
        &output_path,
        "StandardShader" // Module name for generated types
    )
    .context("Failed to generate standard shader bindings")?;

    println!("cargo:rerun-if-changed=build.rs");
    // Rerun-if-changed for the WGSL file is handled inside generate_shader_bindings

    Ok(())
}
