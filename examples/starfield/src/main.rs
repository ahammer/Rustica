//! # Starfield Example
//! 
//! A simple starfield simulation using the Rustica game engine.
//! This example demonstrates the basic usage of the engine,
//! including entity creation, component definition, and plugins.
//!
//! The starfield consists of stars that move across the screen.
//! This serves as a minimal "hello world" demonstrating the engine's
//! architecture and API.

mod physics;
mod input;

use rustica::prelude::*;
use physics::{Position, Velocity, Time, PhysicsConfig};
use input::{create_star_movement_system, create_star_property_system};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use cgmath::{Vector3 as Vec3, Vector4 as Vec4};

// === REGION: COMPONENT DEFINITIONS ===

/// A star component that represents a star in the starfield.
#[derive(Debug, Clone)]
pub struct Star {
    /// Brightness of the star (0.0 to 1.0)
    pub brightness: f32,
    /// Size of the star
    pub size: f32,
    /// Color of the star (optional)
    pub color: Option<[f32; 3]>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            brightness: 0.8,
            size: 1.0,
            color: None,
        }
    }
}

// === REGION: PLUGIN DEFINITION ===

/// The main plugin for the Starfield example.
struct StarfieldPlugin {
    /// Number of stars to create
    star_count: usize,
    /// Physics configuration
    physics_config: PhysicsConfig,
}

impl Default for StarfieldPlugin {
    fn default() -> Self {
        Self {
            star_count: 1000,
            physics_config: PhysicsConfig::default(),
        }
    }
}

impl StarfieldPlugin {
    /// Create a new StarfieldPlugin with custom settings
    pub fn new(star_count: usize, physics_config: PhysicsConfig) -> Self {
        Self {
            star_count,
            physics_config,
        }
    }
    
    /// Spawn stars with random positions and velocities
    fn spawn_stars(&self, world: &mut World) {
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        
        // Generate random stars
        for _ in 0..self.star_count {
            let x = rng.gen_range(0.0..self.physics_config.world_bounds.x);
            let y = rng.gen_range(0.0..self.physics_config.world_bounds.y);
            let z = rng.gen_range(0.0..self.physics_config.world_bounds.z);
            
            let velocity = Velocity {
                value: Vec3::new(
                    rng.gen_range(-0.5..0.5),
                    rng.gen_range(-0.5..0.5),
                    rng.gen_range(-2.0..-0.5),
                ),
                damping: 1.0,
            };
            
            // Vary star properties
            let brightness = rng.gen_range(0.5..1.0);
            let size = rng.gen_range(1.0..5.0);
            
            // Create color variations (white with slight tints)
            let color = if rng.gen_bool(0.7) {
                None // 70% white stars
            } else {
                let r = rng.gen_range(0.7..1.0);
                let g = rng.gen_range(0.7..1.0);
                let b = rng.gen_range(0.7..1.0);
                Some([r, g, b])
            };
            
            let position = Position {
                value: Vec3::new(x, y, z),
            };
            
            let star = Star {
                brightness,
                size,
                color,
            };
            
            // Create a debug star component for rendering
            // This component acts as the descriptor that the render system will use
            // to generate render commands - it doesn't know about rendering details
            let debug_star = DebugStarComponent {
                color: Vec4::new(
                    color.map_or(1.0, |c| c[0]),
                    color.map_or(1.0, |c| c[1]),
                    color.map_or(1.0, |c| c[2]),
                    1.0
                ),
                size,
                brightness,
                visible: true,
            };
            
            // The rendering pipeline will:
            // 1. Find entities with Position and DebugStarComponent
            // 2. Convert them to render commands
            // 3. Process those commands into GPU draw calls
            
            world.spawn()
                .insert(position)
                .insert(velocity)
                .insert(star)
                .insert(debug_star);
        }
        
        println!("Created {} stars", self.star_count);
    }
    
    /// Create a system to update the debug star components based on their position
    fn create_debug_star_update_system() -> impl FnMut(&mut World) {
        use rustica_debug_render::components::DebugStarComponent;
        use rustica_debug_render::command::RenderCommandList;
        use rustica_debug_render::star_renderer::Star;

        move |world| {
            // Create a command list if it doesn't exist
            if world.get_resource::<RenderCommandList>().is_none() {
                world.insert_resource(RenderCommandList::new());
            }
            
            // Query for all entities with Position and DebugStarComponent
            // This is a simplified query since World implementation might not support full query API yet
            let mut entities_to_process = Vec::new();
            
            // Manual "query" - in a complete implementation, we would use World::query
            world.for_each_entity(|entity_id| {
                if let (Some(position), Some(debug_star)) = (
                    world.get::<Position>(entity_id),
                    world.get::<DebugStarComponent>(entity_id)
                ) {
                    if debug_star.visible {
                        entities_to_process.push((entity_id, position.clone(), debug_star.clone()));
                    }
                }
            });
            
            // Process entities and update render commands
            if let Some(command_list) = world.get_resource_mut::<RenderCommandList>() {
                // Clear previous frame's stars
                command_list.clear_stars();
                
                // Add visible stars to the command list
                for (_entity_id, position, debug_star) in entities_to_process {
                    // Convert the position and debug star components to a render star
                    let star = Star::new(
                        position.value,
                        debug_star.color,
                        debug_star.size,
                        debug_star.brightness
                    );
                    
                    // Add the star to the render commands
                    command_list.add_star(star);
                }
                
                println!("Debug star update: {} stars processed", command_list.star_count());
            }
        }
    }
}

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        // Add resources
        app.insert_resource(self.physics_config);
        app.insert_resource(Time::default());
        
        // Register physics systems
        app.add_system(create_position_update_system(), "position_update", Stage::Update);
        app.add_system(create_velocity_update_system(), "velocity_update", Stage::Update);
        app.add_system(create_boundary_wrap_system(), "boundary_wrap", Stage::LateUpdate);
        app.add_system(Self::create_debug_star_update_system(), "debug_star_update", Stage::LateUpdate);
        
        // Register input handling systems
        app.add_system(create_star_movement_system(), "star_movement", Stage::EarlyUpdate);
        app.add_system(create_star_property_system(), "star_property", Stage::EarlyUpdate);
        
        // For now, just spawn stars
        if let Some(world) = app.get_resource_mut::<World>() {
            self.spawn_stars(world);
        }
    }
    
    fn name(&self) -> &str {
        "StarfieldPlugin"
    }
}

// === REGION: SYSTEMS ===

// Our systems are now defined in the physics.rs module

// === REGION: MAIN FUNCTION ===

fn main() {
    // Create a new application
    let mut app = App::new();
    
    // Add the ECS plugin for entity management
    app.add_plugin(EcsPlugin::default());
    
    // Add the render plugin for window
    app.add_plugin(RenderPlugin::default());
    
    // Add the debug render plugin
    app.add_plugin(DebugRenderPlugin::new());
    
    // Configure physics
    let physics_config = PhysicsConfig {
        world_bounds: Vec3::new(1000.0, 800.0, 1000.0),
        wrap_around_bounds: true,
    };
    
    // Add the starfield plugin with 100 stars (reduced for better performance during debug)
    app.add_plugin(StarfieldPlugin::new(100, physics_config));
    
    // Print debug information
    println!("Starting Starfield example - Hello Rustica World!");
    
    if let Some(_world) = app.get_resource::<World>() {
        println!("Starfield initialized with Position/Velocity system");
        println!("Physics configuration:");
        if let Some(config) = app.get_resource::<PhysicsConfig>() {
            println!("  World bounds: {:.1} x {:.1} x {:.1}", 
                config.world_bounds.x, 
                config.world_bounds.y, 
                config.world_bounds.z);
            println!("  Wrap around bounds: {}", config.wrap_around_bounds);
        }
        println!("Running application with window and debug renderer...");
    }
    
    // Run the application with the event loop (this is a blocking call)
    app.run();
}
