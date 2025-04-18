// Animated triangle shader with instanced rendering support
// Supports per-vertex colors, instance transformation, and time uniform for animations

// Uniform buffer for global values
struct Uniforms {
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Vertex shader output
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    // Vertex attributes
    @location(0) position: vec3<f32>,
    @location(1) vertex_color: vec3<f32>,
    
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
    
    // Apply model transformation to vertex position
    out.clip_position = model * vec4<f32>(position, 1.0);
    
    // Apply time-based pulsing to color (optional - this effect is now controlled by the CPU)
    // let pulse = (sin(uniforms.time * 2.0) * 0.2) + 0.8;
    
    // Blend instance color with vertex color
    out.color = vertex_color * instance_color;
    
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
