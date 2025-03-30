// Vertex shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> model: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> view: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform the vertex position
    let world_position = model * vec4<f32>(position, 1.0);
    out.clip_position = projection * view * world_position;
    
    // Pass data to fragment shader
    out.color = color;
    out.normal = normal;
    
    // Calculate world normal for lighting
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * normal);
    out.world_position = world_position.xyz;
    
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting calculation
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 3.0));
    let ambient = 0.3;
    let diffuse = max(dot(in.world_normal, light_dir), 0.0);
    
    // Calculate final color with lighting
    let light_intensity = ambient + diffuse * 0.7;
    let final_color = in.color * light_intensity;
    
    return vec4<f32>(final_color, 1.0);
}
