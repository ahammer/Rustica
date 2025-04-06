// Uniforms
struct TimeUniforms {
    time: f32,
};

@group(0) @binding(0) var<uniform> view_proj_matrix: mat4x4<f32>; // Changed to match Rust struct
@group(0) @binding(1) var<uniform> time_uniform: TimeUniforms; // Renamed to avoid conflict with time built-in

// Vertex shader input/output structures
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, // Included for potential future use
};

// Instance attributes
struct InstanceInput {
    @location(2) model_row0: vec4<f32>,
    @location(3) model_row1: vec4<f32>,
    @location(4) model_row2: vec4<f32>,
    @location(5) model_row3: vec4<f32>,
    @location(6) instance_color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>, // Added to match Rust struct vertex attrs
};

@vertex
fn vs_main(
    vert: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;

    // Reconstruct model matrix from instance data
    let model = mat4x4<f32>(
        instance.model_row0,
        instance.model_row1,
        instance.model_row2,
        instance.model_row3
    );

    // Calculate world position (no wave effect in shader, handled in Rust)
    let world_pos = model * vec4<f32>(vert.position, 1.0);

    // Calculate clip position
    out.clip_position = view_proj_matrix * world_pos; // Use the direct uniform name

    // Pass instance color to fragment shader
    out.color = instance.instance_color;

    // Pass normal through (even if unused by fragment shader)
    out.normal = vert.normal;

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Output the interpolated instance color
    return vec4<f32>(in.color, 1.0);
}
