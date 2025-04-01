// Custom shader with plasma effect and instanced rendering support

// Vertex shader input/output structures
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

// Uniform for time
struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    // Vertex attributes
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
    
    // Instance attributes (model matrix rows)
    @location(3) model_row0: vec4<f32>,
    @location(4) model_row1: vec4<f32>,
    @location(5) model_row2: vec4<f32>,
    @location(6) model_row3: vec4<f32>,
    
    // Instance color
    @location(7) instance_color: vec3<f32>
) -> VertexOutput {
    var out: VertexOutput;
    
    // Reconstruct model matrix from instance data
    let model = mat4x4<f32>(
        model_row0,
        model_row1,
        model_row2,
        model_row3
    );
    
    // Apply optional wave effect on vertices
    var pos = position;
    pos.y += sin(pos.x * 3.0 + uniforms.time) * 0.05;
    
    // Apply model transformation to vertex position
    out.clip_position = model * vec4<f32>(pos, 1.0);
    
    // Combine vertex color with instance color
    out.color = color * instance_color;
    
    // Pass UV coordinates for plasma effect
    out.uv = uv;
    
    return out;
}

// Fragment shader with plasma effect
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Use the UV coordinates to drive the plasma effect
    let uv = in.uv;
    
    // Scale UV coordinates to increase the frequency of the pattern
    let pos = uv * 10.0;
    
    // Combine several sine waves to create a complex plasma pattern
    let v1 = sin(pos.x + uniforms.time);
    let v2 = sin(pos.y + uniforms.time);
    let v3 = sin(pos.x + pos.y + uniforms.time);
    let v4 = sin(sqrt(pos.x * pos.x + pos.y * pos.y) + uniforms.time);
    
    // Sum the sine values
    let value = v1 + v2 + v3 + v4;
    
    // Normalize the combined value from [-4, 4] into [0, 1]
    let plasma = (value + 4.0) / 8.0;
    
    // Map the plasma value to a color palette
    let red   = sin(3.1415 * plasma);
    let green = sin(3.1415 * plasma + 2.0944);  // 120° phase shift
    let blue  = sin(3.1415 * plasma + 4.18879); // 240° phase shift
    
    // Blend plasma color with the vertex/instance color
    let plasma_color = vec3<f32>(red, green, blue);
    let final_color = mix(plasma_color, in.color, 0.3);
    
    return vec4<f32>(final_color, 1.0);
}
