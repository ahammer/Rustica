// PBR Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>,
};

// Camera uniforms
@group(1) @binding(0)
var<uniform> view: mat4x4<f32>;

@group(1) @binding(1)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Get model matrix from push constants or instance buffer
    // For now, we'll use identity matrix
    let model = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );
    
    // Calculate world position
    let world_position = model * vec4<f32>(in.position, 1.0);
    out.world_position = world_position.xyz;
    
    // Calculate clip position
    out.clip_position = projection * view * world_position;
    
    // Transform normal to world space
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * in.normal);
    
    // Pass through texture coordinates and color
    out.tex_coords = in.tex_coords;
    out.color = in.color;
    
    return out;
}
