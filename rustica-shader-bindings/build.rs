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
        .expect("Failed to read generated file");

    // Fix inner attribute issue by converting #![] to #[]
    let fixed_content = content.replace("#![allow", "#[allow");

    // Write the fixed content to the final output file
    let mut output_file = File::create(&final_output_path)
        .expect("Failed to create output file");
    output_file.write_all(fixed_content.as_bytes())
        .expect("Failed to write fixed content");    // Success message
    println!("cargo:warning=Successfully generated bindings for {}", pbr_shader_path.display());
}
