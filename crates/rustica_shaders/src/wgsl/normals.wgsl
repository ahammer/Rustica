// Basic shader that transforms vertices and visualizes normals.

struct CameraUniforms {
    mvp: mat4x4<f32>,
    // Assuming model matrix is applied via instance buffer or another uniform group
    // For simplicity, we'll assume normals are already in world space or use model matrix from instance
    // A proper implementation might need a normal matrix (inverse transpose model)
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

// Instance data (example)
struct InstanceInput {
     @location(3) model_matrix_col_0: vec4<f32>,
     @location(4) model_matrix_col_1: vec4<f32>,
     @location(5) model_matrix_col_2: vec4<f32>,
     @location(6) model_matrix_col_3: vec4<f32>,
     // Add normal matrix if needed
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
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
    // Transform position
    out.clip_position = camera.mvp * model_matrix * vec4<f32>(model.position, 1.0);

    // Transform normal (simplified: assumes no non-uniform scaling in model_matrix)
    // Proper way requires inverse transpose of model_matrix's upper 3x3
    out.world_normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Visualize world normals: map range [-1, 1] to [0, 1] for color
    let normal_color = normalize(in.world_normal) * 0.5 + 0.5;
    return vec4<f32>(normal_color, 1.0);
}
