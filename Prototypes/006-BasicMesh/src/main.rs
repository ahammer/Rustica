use std::sync::Arc;
use cgmath::{Matrix4, Point3, Rad, Vector3};
use rustica_graphics::{Camera, primitives::shapes::cube::create_cube};
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter
};

// Define a custom vertex type with derive macro
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct MeshVertex {
    position: [f32; 3], // location = 0, format = Float32x3
    color: [f32; 3],    // location = 1, format = Float32x3
    normal: [f32; 3],   // location = 2, format = Float32x3
}

// Define an instance struct for instanced rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeInstance {
    model_matrix: [[f32; 4]; 4], // locations 3,4,5,6 (4 rows)
    color: [f32; 3],             // location 7
    _padding: u32,               // For memory alignment
}

impl CubeInstance {
    pub fn new(model_matrix: [[f32; 4]; 4], color: [f32; 3]) -> Self {
        Self {
            model_matrix,
            color,
            _padding: 0,
        }
    }
}

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/mesh_shader.wgsl")]
struct MeshShaderDescriptor {    
    #[vertex_type]
    vertex: MeshVertex,
    
    // Now we don't need the model uniform since it's supplied per-instance
    
    #[uniform(binding = 1)]
    view: Matrix4<f32>,
    
    #[uniform(binding = 2)]
    projection: Matrix4<f32>,
    
    #[uniform(binding = 3)]
    time: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = MeshShaderDescriptor::descriptor();
    
    let mut window = RenderWindow::new("Basic Mesh Demo (Instanced)", 800, 600);
    
    // Register the shader with the render context
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a cube mesh using Graphics factory function
    let cube_mesh = Arc::new(create_cube(0.5)); // Smaller cube size
    
    // Create a mesh adapter with a vertex mapper function
    let mesh_adapter = StandardMeshAdapter::new(cube_mesh, |standard_vertex| {
        MeshVertex {
            position: standard_vertex.position,
            color: standard_vertex.color,
            normal: standard_vertex.normal,
        }
    });
    
    // Set up the frame callback
    window.with_frame_callback(move |canvas| {
        let time = canvas.time().as_secs_f32();
        
        // Set up camera using the Camera struct
        let mut camera = Camera::perspective(800.0 / 600.0);
        camera.look_at_from(
            Point3::new(0.0, 2.0, 5.0), 
            Point3::new(0.0, 0.0, 0.0)
        );
        
        // Get camera matrices
        let camera_matrices = camera.get_render_matrices();
        let view = camera_matrices.view;
        let projection = camera_matrices.projection;
        
        // Get triangles from the mesh adapter
        let triangles = mesh_adapter.to_triangles();
        
        // Create instance data for multiple cubes
        let mut instances = Vec::new();
        
        // Number of cubes in a row
        let grid_size = 3;
        let spacing = 1.5;
        
        for x in 0..grid_size {
            for y in 0..grid_size {
                for z in 0..grid_size {
                    // Calculate position offset to form a 3D grid centered at (0,0,0)
                    let pos_x = (x as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
                    let pos_y = (y as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
                    let pos_z = (z as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
                    
                    // Create a unique rotation speed for each cube
                    let rot_speed_x = 0.3 + (x as f32 * 0.1);
                    let rot_speed_y = 0.4 + (y as f32 * 0.1);
                    let rot_speed_z = 0.5 + (z as f32 * 0.1);
                    
                    // Create individual rotation matrix
                    let rotation = Matrix4::from_angle_x(Rad(time * rot_speed_x)) *
                                   Matrix4::from_angle_y(Rad(time * rot_speed_y)) *
                                   Matrix4::from_angle_z(Rad(time * rot_speed_z));
                    
                    // Create translation matrix
                    let translation = Matrix4::from_translation(Vector3::new(pos_x, pos_y, pos_z));
                    
                    // Combine rotation and translation
                    let model = translation * rotation;
                    
                    // Convert to 2D array format
                    let model_array = [
                        [model.x.x, model.x.y, model.x.z, model.x.w],
                        [model.y.x, model.y.y, model.y.z, model.y.w],
                        [model.z.x, model.z.y, model.z.z, model.z.w],
                        [model.w.x, model.w.y, model.w.z, model.w.w],
                    ];
                    
                    // Create a color based on position
                    let cube_color = [
                        0.2 + 0.6 * (x as f32 / grid_size as f32),
                        0.2 + 0.6 * (y as f32 / grid_size as f32),
                        0.2 + 0.6 * (z as f32 / grid_size as f32),
                    ];
                    
                    // Add to instances
                    instances.push(CubeInstance::new(model_array, cube_color));
                }
            }
        }
        
        // Draw all cubes with a single instanced call
        canvas.draw_with_instances(shader_id)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .colored_instanced_triangles(&triangles, &instances);
    }).run()?;
    
    Ok(())
}
