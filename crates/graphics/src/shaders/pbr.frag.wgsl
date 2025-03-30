// PBR Fragment Shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>,
};

// Material uniforms
@group(0) @binding(0)
var<uniform> material: MaterialUniforms;

// Camera uniforms
@group(1) @binding(2)
var<uniform> camera_position: vec4<f32>;

// Material texture
@group(0) @binding(1)
var diffuse_texture: texture_2d<f32>;

// Material sampler
@group(0) @binding(2)
var diffuse_sampler: sampler;

// Material uniform structure
struct MaterialUniforms {
    diffuse_color: vec4<f32>,
    specular_color: vec4<f32>,
    ambient_factor: f32,
    padding: vec3<f32>,
};

// Constants
const PI: f32 = 3.14159265359;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Get material properties
    let diffuse_color = material.diffuse_color.rgb;
    let specular_color = material.specular_color.rgb;
    let shininess = material.specular_color.a;
    let ambient_factor = material.ambient_factor;
    
    // Sample diffuse texture if available
    var base_color = in.color * diffuse_color;
    
    // Calculate lighting
    let normal = normalize(in.world_normal);
    let view_dir = normalize(camera_position.xyz - in.world_position);
    
    // Directional light (hardcoded for now)
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    
    // Ambient component
    let ambient = ambient_factor * base_color;
    
    // Diffuse component
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = n_dot_l * base_color;
    
    // Specular component (Blinn-Phong)
    let half_dir = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, half_dir), 0.0);
    let specular = pow(n_dot_h, shininess) * specular_color;
    
    // Combine lighting components
    let final_color = ambient + diffuse + specular;
    
    return vec4<f32>(final_color, 1.0);
}
