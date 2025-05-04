use std::env;
use std::path::PathBuf;
use wgsl_bindgen::{GlamWgslTypeMap, WgslBindgenOptionBuilder, WgslTypeSerializeStrategy};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let shader_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("shaders");
    let pbr_shader_path = shader_dir.join("pbr.wgsl");

    // Use expect for clearer error messages if paths are invalid UTF-8
    let pbr_shader_path_str = pbr_shader_path.to_str().expect("Shader path is not valid UTF-8");
    let output_path = out_dir.join("pbr_bindings.rs");

    // Tell cargo to rerun the build script if the shader changes
    println!("cargo:rerun-if-changed={}", pbr_shader_path.display());

    WgslBindgenOptionBuilder::default()
        .workspace_root(shader_dir.to_str().expect("Shader dir is not valid UTF-8")) // Set workspace root
        .add_entry_point(pbr_shader_path_str) // Add the specific shader entry point
        .output(output_path) // Set the output file path directly
        // Specify Bytemuck strategy FIRST
        .serialization_strategy(WgslTypeSerializeStrategy::Bytemuck)
        // THEN set the type map
        .type_map(GlamWgslTypeMap) // Use glam types
        .build()
        .expect("Failed to build wgsl_bindgen options")
        .generate()
        .expect("Unable to generate bindings");

    // Optional: Add a success message for clarity during builds
    println!("cargo:warning=Successfully generated bindings for {}", pbr_shader_path.display());
}
