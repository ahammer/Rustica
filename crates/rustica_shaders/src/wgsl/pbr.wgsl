// Basic PBR structure. Outputs albedo color for now.

struct CameraUniforms {
    mvp: mat4x4<f32>,
    camera_pos: vec3<f32>, // World space camera position
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

// Example Light (simplified)
struct LightUniforms {
    position: vec3<f32>, // World space light position
    color: vec3<f32>,
};
@group(0) @binding(1) var<uniform> light: LightUniforms;


struct MaterialUniforms {
    albedo: vec4<f32>, // Base color + alpha
    metallic: f32,
    roughness: f32,
    // Add emission, etc. if needed
};
@group(1) @binding(0) var<uniform> material: MaterialUniforms;

// Optional textures
@group(1) @binding(1) var albedo_texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

// Instance data (example)
struct InstanceInput {
     @location(3) model_matrix_col_0: vec4<f32>,
     @location(4) model_matrix_col_1: vec4<f32>,
     @location(5) model_matrix_col_2: vec4<f32>,
     @location(6) model_matrix_col_3: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
     let model_matrix = mat4x4<f32>(
        instance.model_matrix_col_0,
        instance.model_matrix_col_1,
        instance.model_matrix_col_2,
        instance.model_matrix_col_3
    );
    let world_pos_4 = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos_4.xyz;
    out.clip_position = camera.mvp * world_pos_4;
    out.world_normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    out.uv = model.uv;
    return out;
}

// Basic fragment shader - just outputs sampled albedo or uniform albedo
// Full PBR lighting calculation is significantly more complex
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample albedo texture if available, otherwise use uniform
    // This requires logic in the Rust code to bind a dummy white texture if no texture is provided
    let albedo_sample = textureSample(albedo_texture, texture_sampler, in.uv);
    let final_albedo = material.albedo * albedo_sample; // Modulate uniform color with texture

    // Placeholder: Just output the final albedo color
    // TODO: Implement PBR lighting calculations (BRDF, Fresnel, etc.)
    return final_albedo;
}
