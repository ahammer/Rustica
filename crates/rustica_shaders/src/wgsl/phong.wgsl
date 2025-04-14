// Basic Phong lighting model with one directional light.

struct CameraUniforms {
    mvp: mat4x4<f32>,
    camera_pos: vec3<f32>, // World space camera position
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

struct LightUniforms {
    direction: vec3<f32>, // World space light direction (pointing towards light source)
    color: vec3<f32>,
    ambient_intensity: f32,
};
@group(0) @binding(1) var<uniform> light: LightUniforms;

struct MaterialUniforms {
    ambient_color: vec3<f32>,
    diffuse_color: vec3<f32>,
    specular_color: vec3<f32>,
    shininess: f32,
};
// Use group 1 for material properties, assuming they might change less often than camera/light
@group(1) @binding(0) var<uniform> material: MaterialUniforms;

// Instance data (example)
struct InstanceInput {
     @location(3) model_matrix_col_0: vec4<f32>,
     @location(4) model_matrix_col_1: vec4<f32>,
     @location(5) model_matrix_col_2: vec4<f32>,
     @location(6) model_matrix_col_3: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
     let model_matrix = mat4x4<f32>(
        instance.model_matrix_col_0,
        instance.model_matrix_col_1,
        instance.model_matrix_col_2,
        instance.model_matrix_col_3
    );
    let world_pos_4 = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos_4.xyz;
    out.clip_position = camera.mvp * world_pos_4;
    // Simplified normal transformation
    out.world_normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(light.direction);
    let view_dir = normalize(camera.camera_pos - in.world_position);

    // Ambient
    let ambient = material.ambient_color * light.ambient_intensity;

    // Diffuse
    let diffuse_strength = max(dot(normal, light_dir), 0.0);
    let diffuse = material.diffuse_color * light.color * diffuse_strength;

    // Specular
    let reflect_dir = reflect(-light_dir, normal);
    let spec_strength = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
    let specular = material.specular_color * light.color * spec_strength;

    let final_color = ambient + diffuse + specular;
    return vec4<f32>(final_color, 1.0);
}
