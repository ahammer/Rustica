// Teapot shader with lighting

// Vertex shader
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
    
    // Transform position to world space
    let world_position = model * vec4<f32>(in.position, 1.0);
    out.world_position = world_position.xyz;
    
    // Transform normal to world space (ignoring non-uniform scaling for simplicity)
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * in.normal);
    
    // Transform position to clip space
    out.clip_position = projection * view * world_position;
    
    // Pass through texture coordinates and color
    out.tex_coords = in.tex_coords;
    out.color = in.color;
    
    return out;
}

// Fragment shader
struct FragmentInput {
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
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
    let view_dir = normalize(vec3<f32>(0.0, 0.0, 5.0) - in.world_position); // Camera at (0,0,5)
    let reflect_dir = reflect(-light_dir, in.world_normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular = specular_strength * spec * light_color;
    
    // Combine lighting with vertex color
    let result = (ambient + diffuse + specular) * in.color;
    
    return vec4<f32>(result, 1.0);
}
