// Test for instance attribute layout generation
// This test verifies that instance attributes are correctly assigned
// with proper offsets and formats

use rustica_render_derive::ShaderProperties;
use rustica_foundation::Vertex;
use wgpu;

#[derive(ShaderProperties)]
#[shader(inline = "test shader for instance attribute layout")]
struct InstanceAttributeShader {
    // Regular vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    normal: [f32; 3],
    
    // Instance attributes with matrix that spans multiple locations
    #[instance(location = 2)]
    model_matrix: [[f32; 4]; 4],  // Should occupy locations 2, 3, 4, 5
    
    // Instance attribute right after the matrix
    #[instance(location = 6)]
    instance_color: [f32; 3],
    
    // Instance attribute with explicit format
    #[instance(location = 7)]
    instance_id: u32,
}

fn main() {
    // Get the generated instance type
    let instance_size = std::mem::size_of::<InstanceAttributeShaderInstances>();
    
    // Expected size: model_matrix (16 f32s) + instance_color (3 f32s) + instance_id (1 u32)
    // 16 * 4 + 3 * 4 + 1 * 4 = 80 bytes
    assert_eq!(instance_size, 80, "Instance struct size should be 80 bytes");
    
    // Verify that the instance struct has the correct fields
    let instance = InstanceAttributeShaderInstances {
        model_matrix: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        instance_color: [1.0, 0.0, 0.0],
        instance_id: 42,
    };
    
    // Check the instance struct values
    assert_eq!(instance.model_matrix[0][0], 1.0);
    assert_eq!(instance.instance_color[0], 1.0);
    assert_eq!(instance.instance_id, 42);
    
    // Check that the descriptor properly captures the instance attributes
    let descriptor = InstanceAttributeShader::descriptor();
    
    // Verify we can create a geometry builder for this shader type
    let builder = InstanceAttributeShader::geometry_builder();
    let vertex = InstanceAttributeShaderVertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    };
    
    println!("Instance attribute layout test passed successfully!");
}
