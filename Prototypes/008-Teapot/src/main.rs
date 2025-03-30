use std::sync::Arc;
use cgmath::{Matrix4, Point3, Rad};
use rustica_graphics::{Camera, primitives::shapes::teapot::create_default_teapot};
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter,    
};

// Vertex type for the teapot
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct TeapotVertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
    color: [f32; 3],
}

// Shader descriptor for the teapot
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/teapot_shader.wgsl")]
struct TeapotShaderDescriptor {
    #[vertex_type]
    vertex: TeapotVertex,
    
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
    // Create shader descriptor
    let shader_descriptor = TeapotShaderDescriptor::descriptor();
    
    // Create window
    let mut window = RenderWindow::new("Utah Teapot Demo", 800, 600);
    
    // Register shader
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create teapot mesh
    let teapot_mesh = Arc::new(create_default_teapot());
    
    // Create mesh adapter
    let mesh_adapter = StandardMeshAdapter::new(teapot_mesh, |v| {
        TeapotVertex {
            position: v.position,
            normal: v.normal,
            tex_coords: v.tex_coords,
            color: v.color,
        }
    });
    
    // Create camera
    let mut camera = Camera::perspective(800.0 / 600.0);
    camera.look_at_from(
        Point3::new(0.0, 1.5, 5.0), 
        Point3::new(0.0, 0.0, 0.0)
    );
    
    // Set up frame callback
    window.with_frame_callback(move |canvas| {
        let time = canvas.time().as_secs_f32();
        
        // Create model matrix with rotation
        let model = Matrix4::from_angle_y(Rad(time * 0.5));
        
        // Get view and projection matrices
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        
        // Draw the teapot
        canvas.draw_with_shader(shader_id)
              .uniform("model", model)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .triangles(&mesh_adapter.to_triangles());
    }).run()?;
    
    Ok(())
}
