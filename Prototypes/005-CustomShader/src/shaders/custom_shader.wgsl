// Vertex shader remains mostly unchanged
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct Uniforms {
    @location(0) time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Optionally, keep the simple wave effect on the vertices
    var pos = in.position;
    pos.y += sin(pos.x * 3.0 + uniforms.time) * 0.1;
    
    out.clip_position = vec4<f32>(pos, 1.0);
    out.color = in.color;
    out.uv = in.uv;
    return out;
}

// Fragment shader with plasma effect
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Use the UV coordinates to drive the plasma effect.
    let uv = in.uv;
    
    // Scale UV coordinates to increase the frequency of the pattern.
    let pos = uv * 10.0;
    
    // Combine several sine waves to create a complex plasma pattern.
    let v1 = sin(pos.x + uniforms.time);
    let v2 = sin(pos.y + uniforms.time);
    let v3 = sin(pos.x + pos.y + uniforms.time);
    let v4 = sin(sqrt(pos.x * pos.x + pos.y * pos.y) + uniforms.time);
    
    // Sum the sine values.
    let value = v1 + v2 + v3 + v4;
    
    // Normalize the combined value from [-4, 4] into [0, 1].
    let plasma = (value + 4.0) / 8.0;
    
    // Map the plasma value to a color palette.
    let red   = sin(3.1415 * plasma);
    let green = sin(3.1415 * plasma + 2.0944);  // 120° phase shift
    let blue  = sin(3.1415 * plasma + 4.18879); // 240° phase shift
    
    return vec4<f32>(vec3<f32>(red, green, blue), 1.0);
}
