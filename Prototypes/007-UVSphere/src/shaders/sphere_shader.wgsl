// Vertex shader with camera, projection, and instanced rendering support
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

// No model uniform anymore since it comes from instance data

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
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
    
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
    
    // Apply model-view-projection transformation
    let model_position = model * vec4<f32>(position, 1.0);
    out.world_position = model_position.xyz;
    out.clip_position = projection * view * model_position;
    
    // Transform normal to world space (ignoring non-uniform scaling for simplicity)
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * normal);
    
    // Blend vertex color with instance color
    out.color = color * instance_color;
    out.tex_coords = tex_coords;
    
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
    
    // Calculate specular light
    let view_dir = normalize(vec3<f32>(0.0, 0.0, 15.0) - in.world_position);
    let reflect_dir = reflect(-light_dir, in.world_normal);
    let specular_strength = 0.5;
    let shininess = 32.0;
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular = specular_strength * spec;
    
    // Create a time-based highlight effect for sun (assuming it's instance 0)
    let sun_glow = max(0.0, 1.0 - length(in.world_position));
    let sun_pulse = sun_glow * abs(sin(time * 0.5)) * 0.3;
    
    // Combine ambient and diffuse lighting with vertex/texture color
    let lighting = ambient + diffuse + specular + sun_pulse;
    let final_color = mix(in.color, tex_color, 0.3) * lighting;
    
    return vec4<f32>(final_color, 1.0);
}
