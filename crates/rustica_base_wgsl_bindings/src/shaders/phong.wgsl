// Basic Phong Lighting Shader

// Vertex Input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

// Vertex Output
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

// Uniforms
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct ModelUniform {
    model: mat4x4<f32>,
    normal_transform: mat3x3<f32>, // Inverse transpose of model matrix (upper 3x3)
};
@group(1) @binding(0)
var<uniform> model: ModelUniform;

struct LightUniform {
    position: vec3<f32>,
    color: vec3<f32>,
};
@group(2) @binding(0)
var<uniform> light: LightUniform;

struct MaterialUniform {
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
    shininess: f32,
};
@group(2) @binding(1)
var<uniform> material: MaterialUniform;


// Vertex Shader
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos4 = model.model * vec4<f32>(in.position, 1.0);
    out.world_position = world_pos4.xyz;
    out.clip_position = camera.view_proj * world_pos4;
    out.world_normal = normalize(model.normal_transform * in.normal);
    return out;
}

// Fragment Shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Ambient
    let ambient_color = material.ambient * light.color;

    // Diffuse
    let light_dir = normalize(light.position - in.world_position);
    let diff = max(dot(in.world_normal, light_dir), 0.0);
    let diffuse_color = material.diffuse * diff * light.color;

    // Specular
    let view_dir = normalize(camera.position - in.world_position);
    let reflect_dir = reflect(-light_dir, in.world_normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
    let specular_color = material.specular * spec * light.color;

    let result = ambient_color + diffuse_color + specular_color;
    return vec4<f32>(result, 1.0);
}
