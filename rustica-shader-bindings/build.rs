use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use wgsl_bindgen::{GlamWgslTypeMap, Regex, WgslBindgenOptionBuilder, WgslTypeSerializeStrategy};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let shader_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("shaders");
    let pbr_shader_path = shader_dir.join("pbr.wgsl");

    // Use expect for clearer error messages if paths are invalid UTF-8
    let pbr_shader_path_str = pbr_shader_path.to_str().expect("Shader path is not valid UTF-8");
    let raw_output_path = out_dir.join("raw_pbr_bindings.rs");
    let final_output_path = out_dir.join("pbr_bindings.rs");

    // Tell cargo to rerun the build script if the shader changes
    println!("cargo:rerun-if-changed={}", pbr_shader_path.display());

    // Generate bindings to a temporary file first
    WgslBindgenOptionBuilder::default()
        .workspace_root(shader_dir.to_str().expect("Shader dir is not valid UTF-8"))
        .add_entry_point(pbr_shader_path_str)
        .skip_hash_check(true)
        .skip_header_comments(true) 
        .serialization_strategy(WgslTypeSerializeStrategy::Bytemuck)
        .type_map(GlamWgslTypeMap)
        .add_custom_padding_field_regexp(Regex::new("_pad.*").unwrap())
        .output(&raw_output_path)
        .build()
        .expect("Failed to build wgsl_bindgen options")
        .generate()
        .expect("Unable to generate bindings");

    // Read the generated file
    let mut content = String::new();
    File::open(&raw_output_path)
        .expect("Failed to open generated file")
        .read_to_string(&mut content)
        .expect("Failed to read generated file");    // Fix inner attribute issue by converting #![] to #[]
    let mut fixed_content = content.replace("#![allow", "#[allow");

    // Make layout assert constants public so tests can reference them
    fixed_content = fixed_content.replace(
        "const WGSL_BASE_TYPE_ASSERTS: () = {",
        "#[allow(dead_code)] pub const WGSL_BASE_TYPE_ASSERTS: () = {"
    );
    fixed_content = fixed_content.replace(
        "const PBR_CAMERA_UNIFORM_ASSERTS: () = {",
        "#[allow(dead_code)] pub const PBR_CAMERA_UNIFORM_ASSERTS: () = {"
    );
    fixed_content = fixed_content.replace(
        "const PBR_MODEL_UNIFORM_ASSERTS: () = {",
        "#[allow(dead_code)] pub const PBR_MODEL_UNIFORM_ASSERTS: () = {"
    );
    fixed_content = fixed_content.replace(
        "const PBR_MATERIAL_UNIFORM_ASSERTS: () = {",
        "#[allow(dead_code)] pub const PBR_MATERIAL_UNIFORM_ASSERTS: () = {"
    );
    
    // Add #[allow(dead_code)] to ShaderEntry implementation methods
    fixed_content = fixed_content.replace(
        "impl ShaderEntry {",
        "impl ShaderEntry {\n    #[allow(dead_code)]"
    );
    fixed_content = fixed_content.replace(
        "    pub fn create_shader_module_embed_source(",
        "    #[allow(dead_code)]\n    pub fn create_shader_module_embed_source("
    );
    
    // Fix other warnings in the generated code
    fixed_content = fixed_content.replace(
        "use super::{_root, _root::*};",
        "use super::_root::*;"
    );
    
    // Convert doc comments to regular comments in wgpu bind group entries
    fixed_content = fixed_content.replace(
        "/// @binding(",
        "// @binding("
    );

    // Write the fixed content to the final output file
    let mut output_file = File::create(&final_output_path)
        .expect("Failed to create output file");
    output_file.write_all(fixed_content.as_bytes())
        .expect("Failed to write fixed content");// Success message
}
