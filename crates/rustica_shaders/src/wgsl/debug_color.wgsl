// Basic shader that transforms vertices and passes vertex color through.

struct CameraUniforms {
    mvp: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>, // Expect vertex color
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.mvp * vec4<f32>(model.position, 1.0);
    out.color = model.color; // Pass color to fragment shader
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0); // Output interpolated vertex color
}
