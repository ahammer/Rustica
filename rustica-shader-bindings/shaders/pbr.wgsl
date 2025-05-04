// Vertex shader inputs
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// Vertex shader outputs / Fragment shader inputs
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// Uniforms accessible by both stages (or potentially just one)
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>, // Camera world position
};

struct ModelUniform {
    model: mat4x4<f32>,
    normal_transform: mat3x3<f32>, // Often inverse transpose of model matrix
};

// Material properties - starting simple
struct MaterialUniform {
    base_color_factor: vec4<f32>,
    metallic_factor: f32,
    roughness_factor: f32,
    // Textures will be added later
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<uniform> model: ModelUniform;
@group(2) @binding(0) var<uniform> material: MaterialUniform;
// @group(2) @binding(1) var base_color_texture: texture_2d<f32>;
// @group(2) @binding(2) var base_color_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos4 = model.model * vec4<f32>(in.position, 1.0);
    out.world_position = world_pos4.xyz;
    out.clip_position = camera.view_proj * world_pos4;
    out.normal = normalize(model.normal_transform * in.normal);
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // For now, just output the base color factor.
    // PBR lighting calculations will be added incrementally.
    return material.base_color_factor;
}
