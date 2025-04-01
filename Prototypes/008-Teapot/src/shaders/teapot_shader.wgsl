// Teapot shader with lighting and instanced rendering support

// Vertex shader output
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>,
};

// No model uniform anymore - comes from instance data
@group(0) @binding(1)
var<uniform> view: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> projection: mat4x4<f32>;

@group(0) @binding(3)
var<uniform> time: f32;

@vertex
fn vs_main(
    // Vertex attributes
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) vertex_color: vec3<f32>,
    
    // Instance attributes (model matrix rows)
    @location(4) model_row0: vec4<f32>,
    @location(5) model_row1: vec4<f32>,
    @location(6) model_row2: vec4<f32>,
    @location(7) model_row3: vec4<f32>,
    
    // Instance color
    @location(8) instance_color: vec3<f32>
) -> VertexOutput {
    var out: VertexOutput;
    
    // Reconstruct model matrix from instance data
    let model = mat4x4<f32>(
        model_row0,
        model_row1,
        model_row2,
        model_row3
    );
    
    // Transform position to world space
    let world_position = model * vec4<f32>(position, 1.0);
    out.world_position = world_position.xyz;
    
    // Transform normal to world space (ignoring non-uniform scaling for simplicity)
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * normal);
    
    // Transform position to clip space
    out.clip_position = projection * view * world_position;
    
    // Pass through texture coordinates and blend vertex/instance colors
    out.tex_coords = tex_coords;
    out.color = vertex_color * instance_color;
    
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Light properties
    let light_position = vec3<f32>(5.0 * sin(time * 0.5), 5.0, 5.0 * cos(time * 0.5));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let ambient_strength = 0.2;
    let specular_strength = 0.5;
    let shininess = 32.0;
    
    // Calculate ambient lighting
    let ambient = ambient_strength * light_color;
    
    // Calculate diffuse lighting
    let light_dir = normalize(light_position - in.world_position);
    let diff = max(dot(in.world_normal, light_dir), 0.0);
    let diffuse = diff * light_color;
    
    // Calculate specular lighting
    let view_position = vec3<f32>(0.0, 3.0, 10.0); // Match camera position in main.rs
    let view_dir = normalize(view_position - in.world_position);
    let reflect_dir = reflect(-light_dir, in.world_normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular = specular_strength * spec * light_color;
    
    // Add subtle rim lighting effect
    let rim_power = 3.0;
    let rim_strength = 0.3;
    let rim = rim_strength * pow(1.0 - max(dot(in.world_normal, view_dir), 0.0), rim_power);
    let rim_color = vec3<f32>(0.3, 0.3, 0.6) * rim;
    
    // Combine lighting with vertex/instance color
    let result = (ambient + diffuse + specular) * in.color + rim_color;
    
    return vec4<f32>(result, 1.0);
}
