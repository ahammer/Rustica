// Placeholder for toon.wgsl

// Basic Toon (Cel) shading with one directional light.

struct CameraUniforms {
    mvp: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

struct LightUniforms {
    direction: vec3<f32>, // World space light direction (pointing towards light source)
    color: vec3<f32>,
};
@group(0) @binding(1) var<uniform> light: LightUniforms;

struct MaterialUniforms {
    base_color: vec3<f32>,
    step_threshold: f32, // e.g., 0.5 - threshold for light/dark transition
    // Add more steps/colors if desired
};
@group(1) @binding(0) var<uniform> material: MaterialUniforms;

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
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
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
    out.clip_position = camera.mvp * world_pos_4;
    out.world_normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(light.direction);

    // Calculate diffuse intensity
    let NdotL = max(dot(normal, light_dir), 0.0);

    // Apply threshold (simple 2-tone)
    var toon_intensity: f32;
    if (NdotL > material.step_threshold) {
        toon_intensity = 1.0; // Bright
    } else {
        toon_intensity = 0.5; // Darker (adjust as needed)
    }

    let final_color = material.base_color * light.color * toon_intensity;
    return vec4<f32>(final_color, 1.0);
}
