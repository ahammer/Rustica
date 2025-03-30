use std::sync::Arc;
use cgmath::{Matrix4, Point3, Rad};
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

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/mesh_shader.wgsl")]
struct MeshShaderDescriptor {    
    #[vertex_type]
    vertex: MeshVertex,
    
    #[uniform(binding = 0)]
    model: Matrix4<f32>,
    
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
    
    let mut window = RenderWindow::new("Basic Mesh Demo", 800, 600);
    
    // Register the shader with the render context
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a cube mesh using Graphics factory function
    let cube_mesh = Arc::new(create_cube(1.0));
    
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
        
        // Create model matrix with rotation
        let model = Matrix4::from_angle_x(Rad(time * 0.5)) *
                    Matrix4::from_angle_y(Rad(time * 0.7)) *
                    Matrix4::from_angle_z(Rad(time * 0.3));
        
        // Set up camera using the Camera struct
        let mut camera = Camera::perspective(800.0 / 600.0);
        camera.look_at_from(
            Point3::new(0.0, 0.0, 3.0), 
            Point3::new(0.0, 0.0, 0.0)
        );
        
        // Get camera matrices
        let camera_matrices = camera.get_render_matrices();
        let view = camera_matrices.view;
        let projection = camera_matrices.projection;
        
        // Get triangles from the mesh adapter
        let triangles = mesh_adapter.to_triangles();       
        
        // Draw the mesh with the shader
        canvas.draw_with_shader(shader_id)
              .uniform("model", model)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .triangles(&triangles);
    }).run()?;
    
    Ok(())
}
