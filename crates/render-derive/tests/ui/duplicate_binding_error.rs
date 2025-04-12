use rustica_render_derive::ShaderProperties;

#[derive(ShaderProperties)]
#[shader(inline = "")] // Minimal shader source
struct DuplicateBindingShader {
    #[uniform(binding = 1)]
    uniform_a: [[f32; 4]; 4],

    #[uniform(binding = 1)] // Error: Duplicate binding '1'
    uniform_b: [f32; 4],
}

fn main() {} // Required for trybuild
