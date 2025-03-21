//! Render systems for the debug renderer.
//!
//! This module provides systems that query for debug render components
//! and generate render commands for the debug renderer.

use cgmath::{Vector2 as Vec2, Vector3 as Vec3, Vector4 as Vec4};
use rustica_ecs::world::World;
use crate::command::RenderCommandList;
use crate::components::{DebugRenderComponent, DebugStarComponent};

use crate::star_renderer::Star;

/// System to generate render commands from debug star components.
///
/// This system queries for entities with both Position and DebugStarComponent,
/// and generates render commands for them.
pub fn debug_star_render_system(world: &mut World) {
    // Find all entities with a DebugStarComponent and convert to render commands
    
    // Get the command list from the world
    let _command_list = world.get_resource::<RenderCommandList>()
        .expect("RenderCommandList resource not found");
    
    // TEMPORARY: Commented out until we have a proper query API
    // Instead of using for_each_entity which doesn't exist yet
    
    // // Iterate through entities with star components and add them to the command list
    // world.for_each_entity(|entity_id| {
    //     if let Some(star) = world.get_component::<DebugStarComponent>(entity_id) {
    //         // Convert star component to render command
    //         // This isolates rendering details from gameplay logic
    //         
    //         let pos = star.position;
    //         let color = star.color;
    //         let size = star.size;
    //         
    //         // Create a render command and add it to the list
    //         let cmd = RenderCommand::DrawStar {
    //             position: pos,
    //             color,
    //             size,
    //         };
    //         
    //         // Add the command to the list
    //         // This is unsafe because we're mutating a resource while iterating
    //         // A proper implementation would collect commands first, then add them
    //         unsafe {
    //             command_list.add_command(cmd);
    //         }
    //     }
    // });
    
    println!("Star rendering disabled until entity iteration is implemented");
}

/// System to process render commands and draw them using the debug renderer.
///
/// This system takes the render command list and uses the debug renderer
/// to draw all the commands.
pub fn debug_render_process_system(world: &mut World) {
    // CONCEPTUAL IMPLEMENTATION:
    //
    // 1. Get required resources
    // let renderer = world.get_resource::<DebugRendererResource>();
    // let command_list = world.get_resource::<RenderCommandList>();
    // if renderer is None or command_list is None, return
    //
    // 2. Begin a new frame
    // let frame = renderer.begin_frame();
    // 
    // 3. Clear the screen
    // renderer.clear([0.0, 0.0, 0.05, 1.0], &frame);
    // 
    // 4. Process all point commands
    // for point_cmd in &command_list.point_commands {
    //     // Convert to screen space and draw
    //     renderer.draw_point(point_cmd, &frame);
    // }
    // 
    // 5. Process all line commands
    // for line_cmd in &command_list.line_commands {
    //     renderer.draw_line(line_cmd, &frame);
    // }
    // 
    // 6. Process all rectangle commands
    // for rect_cmd in &command_list.rect_commands {
    //     renderer.draw_rect(rect_cmd, &frame);
    // }
    // 
    // 7. End the frame
    // renderer.end_frame(frame);
}
