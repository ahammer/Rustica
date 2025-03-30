// Vertex shader with camera and projection support
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> model: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> view: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> projection: mat4x4<f32>;

@group(0) @binding(3)
var<uniform> time: f32;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply model-view-projection transformation
    let model_position = model * vec4<f32>(in.position, 1.0);
    out.world_position = model_position.xyz;
    out.clip_position = projection * view * model_position;
    
    // Transform normal to world space (ignoring non-uniform scaling for simplicity)
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * in.normal);
    
    // Pass color and texture coordinates to fragment shader
    out.color = in.color;
    out.tex_coords = in.tex_coords;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting calculation
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let ambient = 0.2;
    let diffuse = max(dot(in.world_normal, light_dir), 0.0);
    
    // Add a subtle color variation based on texture coordinates
    let tex_color = vec3<f32>(
        in.tex_coords.x,
        in.tex_coords.y,
        (in.tex_coords.x + in.tex_coords.y) * 0.5
    );
    
    // Combine ambient and diffuse lighting with vertex color and texture influence
    let lighting = ambient + diffuse;
    let final_color = mix(in.color, tex_color, 0.3) * lighting;
    
    return vec4<f32>(final_color, 1.0);
}
