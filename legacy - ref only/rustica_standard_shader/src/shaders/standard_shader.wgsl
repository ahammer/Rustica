// Standard shader for the Rustica engine

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct InstanceInput {
    @location(4) model_matrix_0: vec4<f32>,
    @location(5) model_matrix_1: vec4<f32>,
    @location(6) model_matrix_2: vec4<f32>,
    @location(7) model_matrix_3: vec4<f32>,
    @location(8) instance_color: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> view: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> projection: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> time: f32;

@group(0) @binding(3)
var<uniform> _padding: vec3<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) instance_color: vec3<f32>,
    @location(4) uv: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var output: VertexOutput;
    
    // Reconstruct model matrix from instance input
    let model = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    
    // Calculate world position
    let world_position = model * vec4<f32>(vertex.position, 1.0);
    
    // Calculate final position - Updated to use separate uniform variables
    output.clip_position = projection * view * world_position;
    
    // Transform normal to world space
    output.world_normal = normalize((model * vec4<f32>(vertex.normal, 0.0)).xyz);
    
    // Pass through other attributes
    output.world_position = world_position.xyz;
    output.color = vertex.color;
    output.instance_color = instance.instance_color;
    output.uv = vertex.uv;
    
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting calculation
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 3.0));
    let normal = normalize(in.world_normal);
    
    // Calculate diffuse component
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Calculate ambient component
    let ambient = 0.2;
    
    // Combine lighting with vertex color and instance color
    let lighting = ambient + diffuse;
    let combined_color = in.color * in.instance_color;
    
    // Add a simple animation effect based on time - Updated to use direct time variable
    let time_effect = sin(time + in.world_position.x + in.world_position.z) * 0.1 + 0.9;
    
    return vec4<f32>(combined_color * lighting * time_effect, 1.0);
}
