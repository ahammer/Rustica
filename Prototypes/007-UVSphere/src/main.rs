use std::sync::Arc;
use cgmath::{Matrix4, Point3, Vector3, Rad, perspective, Deg};
use rustica_graphics::{Camera, primitives::shapes::sphere::create_default_sphere};
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter,    
};

// Minimal vertex type for the sphere
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct SphereVertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

// Minimal shader descriptor
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/sphere_shader.wgsl")]
struct SphereShaderDescriptor {
    #[vertex_type]
    vertex: SphereVertex,
    
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
    // Minimal demo setup
    let shader_descriptor = SphereShaderDescriptor::descriptor();
    let mut window = RenderWindow::new("UV Sphere Demo", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a UV sphere mesh using the graphics crate implementation
    let sphere_mesh = Arc::new(create_default_sphere());
    
    // Create a mesh adapter
    let mesh_adapter = StandardMeshAdapter::new(sphere_mesh, |v| {
        SphereVertex {
            position: v.position,
            color: v.color,
            normal: v.normal,
            tex_coords: v.tex_coords,
        }
    });
    
    let mut camera = Camera::perspective(800.0 / 600.0);
    camera.look_at_from(
        Point3::new(0.0, 0.0, 10.0), 
        Point3::new(0.0, 0.0, 0.0)
    );


    // Set up the frame callback with minimal rendering code
    window.with_frame_callback(move |canvas| {
        let time = canvas.time().as_secs_f32();
        
        
        // Simple rotation for the model
        let model = Matrix4::from_angle_y(Rad(time * 0.5));
        
        // Get view and projection matrices from camera
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        
        // Draw the sphere
        canvas.draw_with_shader(shader_id)
              .uniform("model", model)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .triangles(&mesh_adapter.to_triangles());
    }).run()?;
    
    Ok(())
}
