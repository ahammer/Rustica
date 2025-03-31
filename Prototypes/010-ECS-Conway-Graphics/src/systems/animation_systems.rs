// Animation systems for Conway's Game of Life

use rustica_ecs::prelude::*;
use rustica_conway::prelude::*;
use cgmath::{Point3, Vector3};

use crate::components::{CellVisual, CameraState};

/// System for animating cell visuals
pub struct VisualAnimationSystem {
    pub transition_duration: f32, // How long transitions take in seconds
    pub delta_time: f32,          // Time since last frame
}

impl System for VisualAnimationSystem {
    fn run(&self, world: &mut World) {
        // PHASE 1: Collect all entity IDs first
        let mut cell_entities = Vec::new();
        {
            let cell_query = world.query_one::<CellState>();
            for (entity, _) in cell_query {
                cell_entities.push(entity);
            }
        }
        
        // PHASE 2: Create visuals for cells that don't have them
        for &entity in &cell_entities {
            let has_visual = world.get_component::<CellVisual>(entity).is_some();
            if !has_visual {
                world.add_component(entity, CellVisual::default());
            }
        }
        
        // PHASE 3: Update visual targets based on cell states
        // First collect the current state of all cells
        struct CellUpdateInfo {
            entity: Entity,
            is_alive: bool,
        }
        
        let mut updates = Vec::new();
        for &entity in &cell_entities {
            if let Some(state) = world.get_component::<CellState>(entity) {
                updates.push(CellUpdateInfo {
                    entity,
                    is_alive: state.alive,
                });
            }
        }
        
        // Now apply the updates
        for update in &updates {
            if let Some(visual) = world.get_component_mut::<CellVisual>(update.entity) {
                if update.is_alive {
                    // Cell is alive - should be full scale and green
                    if visual.target_scale != 1.0 || visual.target_color != [0.2, 0.8, 0.3] {
                        visual.target_scale = 1.0;
                        visual.target_color = [0.2, 0.8, 0.3]; // Green
                        visual.is_transitioning = true;
                        visual.transition_time = 0.0;
                    }
                } else {
                    // Cell is dead - should be small scale and grey
                    if visual.target_scale != 0.2 || visual.target_color != [0.5, 0.5, 0.5] {
                        visual.target_scale = 0.2;
                        visual.target_color = [0.5, 0.5, 0.5]; // Grey
                        visual.is_transitioning = true;
                        visual.transition_time = 0.0;
                    }
                }
            }
        }
        
        // PHASE 4: Animate visuals
        let mut visual_entities = Vec::new();
        {
            let visual_query = world.query_one::<CellVisual>();
            for (entity, _) in visual_query {
                visual_entities.push(entity);
            }
        }
        
        // Process all animations
        for &entity in &visual_entities {
            if let Some(visual) = world.get_component_mut::<CellVisual>(entity) {
                if visual.is_transitioning {
                    visual.transition_time += self.delta_time;
                    let t = (visual.transition_time / self.transition_duration).min(1.0);
                    
                    // Smooth easing function
                    let smooth_t = t * t * (3.0 - 2.0 * t); // Smoothstep interpolation
                    
                    // Interpolate scale
                    visual.scale = (1.0 - smooth_t) * visual.scale + smooth_t * visual.target_scale;
                    
                    // Interpolate color
                    visual.color[0] = (1.0 - smooth_t) * visual.color[0] + smooth_t * visual.target_color[0];
                    visual.color[1] = (1.0 - smooth_t) * visual.color[1] + smooth_t * visual.target_color[1];
                    visual.color[2] = (1.0 - smooth_t) * visual.color[2] + smooth_t * visual.target_color[2];
                    
                    // Check if animation is complete
                    if visual.transition_time >= self.transition_duration {
                        visual.scale = visual.target_scale;
                        visual.color = visual.target_color;
                        visual.is_transitioning = false;
                    }
                }
            }
        }
    }
}

/// System for animating camera position and target
pub struct CameraAnimationSystem {
    pub transition_duration: f32, // How long transitions take in seconds
    pub delta_time: f32,          // Time since last frame
    pub grid_width: usize,        // Grid dimensions for calculating positions
    pub grid_height: usize,
    pub cube_size: f32,
    pub spacing: f32,
}

impl System for CameraAnimationSystem {
    fn run(&self, world: &mut World) {
        // PHASE 1: Find or create a camera entity
        let mut camera_entity = Entity::invalid();
        
        // First try to find existing camera entity
        {
            let camera_query = world.query_one::<CameraState>();
            for (entity, _) in camera_query {
                camera_entity = entity;
                break; // Just take the first one
            }
        }
        
        // If no camera found, create a new one
        if camera_entity == Entity::invalid() {
            camera_entity = world.create_entity().build();
            
            // Initialize with default state
            world.add_component(camera_entity, CameraState::default());
        }
        
        // PHASE 2: Collect data for calculating camera target
        struct CellPosition {
            pos_x: f32,
            pos_z: f32,
        }
        
        let mut cell_positions = Vec::new();
        {
            let cells_query = world.query_two::<Position, CellState>();
            for (_, (pos, state)) in cells_query {
                if state.alive {
                    // Convert grid position to world position
                    let cell_size = self.cube_size + self.spacing;
                    let grid_width_f32 = self.grid_width as f32;
                    let grid_height_f32 = self.grid_height as f32;
                    
                    let pos_x = (pos.x as f32 - grid_width_f32 / 2.0) * cell_size;
                    let pos_z = (pos.y as f32 - grid_height_f32 / 2.0) * cell_size;
                    
                    cell_positions.push(CellPosition { pos_x, pos_z });
                }
            }
        }
        
        // Calculate median position of cells (or default to center if no cells)
        let count = cell_positions.len();
        let (median_x, median_z) = if count > 0 {
            // Sort cells by x and z separately to find median
            let mut xs: Vec<f32> = cell_positions.iter().map(|p| p.pos_x).collect();
            let mut zs: Vec<f32> = cell_positions.iter().map(|p| p.pos_z).collect();
            
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            zs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            let mid = count / 2;
            let median_x = if count % 2 == 0 {
                (xs[mid-1] + xs[mid]) / 2.0
            } else {
                xs[mid]
            };
            
            let median_z = if count % 2 == 0 {
                (zs[mid-1] + zs[mid]) / 2.0
            } else {
                zs[mid]
            };
            
            (median_x, median_z)
        } else {
            (0.0, 0.0) // Default to center if no cells
        };
        
        // Target the median position with smoothing
        let target_look_at = Point3::new(median_x, 0.0, median_z);
        
        // Get camera state and update orbit
        if let Some(camera_state) = world.get_component_mut::<CameraState>(camera_entity) {
            // Update orbit angle with smoothly varying speed
            // The sin function creates a smooth acceleration and deceleration effect
            let orbit_speed_variation = 0.1 * (camera_state.orbit_angle.sin() * 0.5 + 0.5);
            let adjusted_orbit_speed = camera_state.orbit_speed + orbit_speed_variation;
            camera_state.orbit_angle += adjusted_orbit_speed * self.delta_time;
            
            // Ensure orbit_angle stays within 0 to 2π
            if camera_state.orbit_angle > std::f32::consts::PI * 2.0 {
                camera_state.orbit_angle -= std::f32::consts::PI * 2.0;
            }
            
            // Calculate new orbit position with smooth height variation
            // Use sine function for smooth height oscillation
            let height_variation = camera_state.orbit_height * 0.15 * 
                (camera_state.orbit_angle * 2.0).sin();
                
            let orbit_x = median_x + camera_state.orbit_radius * camera_state.orbit_angle.cos();
            let orbit_y = camera_state.orbit_height + height_variation;
            let orbit_z = median_z + camera_state.orbit_radius * camera_state.orbit_angle.sin();
            
            // Set target position with eased trajectory
            let target_position = Point3::new(orbit_x, orbit_y, orbit_z);
            
            // Advanced smoothing between current and target positions
            let dt = self.delta_time;
            
            // PART 1: Update camera position with spring physics
            // ------------------------------------------------
            
            // Convert Points to Vectors for calculations
            let current_pos = Vector3::new(
                camera_state.position.x, 
                camera_state.position.y, 
                camera_state.position.z
            );
            
            let target_pos = Vector3::new(
                target_position.x,
                target_position.y,
                target_position.z
            );
            
            // Calculate displacement vector
            let displacement = target_pos - current_pos;
            
            // Calculate spring force with cubic easing for smoother motion
            // This applies more force when further away and less when close
            let distance = displacement.magnitude();
            let easing_factor = (distance / 10.0).min(1.0); // Normalize to 0-1 range
            let cubic_easing = easing_factor * easing_factor * (3.0 - 2.0 * easing_factor); // Smooth cubic interpolation
            
            let adjusted_stiffness = camera_state.spring_stiffness * cubic_easing;
            let spring_force = displacement * adjusted_stiffness;
            
            // Critical damping for optimal smoothness
            // This formula creates perfect non-oscillating motion
            let critical_damping = 2.0 * (camera_state.spring_stiffness * 1.0).sqrt();
            let damping_factor = camera_state.spring_damping * critical_damping;
            
            let damping_force = camera_state.velocity * -damping_factor;
            
            // Net force with additional motion smoothing
            let net_force = spring_force + damping_force;
            
            // Update velocity using verlet integration for more stability
            let old_velocity = camera_state.velocity;
            camera_state.velocity = camera_state.velocity + net_force * dt;
            
            // Use velocity verlet method for position update (more stable than Euler)
            let avg_velocity = (old_velocity + camera_state.velocity) * 0.5;
            let new_pos = current_pos + avg_velocity * dt;
            
            // Update camera position 
            camera_state.position = Point3::new(new_pos.x, new_pos.y, new_pos.z);
            
            // PART 2: Update camera target with separate spring physics
            // --------------------------------------------------------
            
            // Convert target points to vectors
            let current_target = Vector3::new(
                camera_state.target.x,
                camera_state.target.y,
                camera_state.target.z
            );
            
            let desired_target = Vector3::new(
                target_look_at.x,
                target_look_at.y,
                target_look_at.z
            );
            
            // Calculate displacement for target
            let target_displacement = desired_target - current_target;
            
            // Calculate spring force for target with much gentler easing
            // Using a quadratic ease-out function for extra smoothness
            let target_distance = target_displacement.magnitude();
            let target_easing = 1.0 - (1.0 - (target_distance / 15.0).min(1.0)).powi(2);
            
            // Apply very gentle spring force to target
            let target_adjusted_stiffness = camera_state.target_spring_stiffness * target_easing;
            let target_spring_force = target_displacement * target_adjusted_stiffness;
            
            // Calculate critical damping for target (typically higher to prevent wobble)
            let target_critical_damping = 2.0 * (camera_state.target_spring_stiffness * 1.0).sqrt();
            let target_damping_factor = camera_state.target_spring_damping * target_critical_damping;
            
            // Apply damping to target velocity
            let target_damping_force = camera_state.target_velocity * -target_damping_factor;
            
            // Net force for target
            let target_net_force = target_spring_force + target_damping_force;
            
            // Update target velocity using verlet integration
            let old_target_velocity = camera_state.target_velocity;
            camera_state.target_velocity = camera_state.target_velocity + target_net_force * dt;
            
            // Use velocity verlet for target position update
            // Using smaller substeps for greater stability
            let substeps = 5; // More substeps for target for extra smoothness
            let sub_dt = dt / substeps as f32;
            
            let mut new_target_pos = current_target;
            for _ in 0..substeps {
                let avg_target_velocity = (old_target_velocity + camera_state.target_velocity) * 0.5;
                new_target_pos = new_target_pos + avg_target_velocity * sub_dt;
            }
            
            // Update camera target with a weighted blend for extra stability
            // This further reduces any potential jitter
            let blend_factor = 0.85; // 85% new position, 15% old position
            let final_target = current_target * (1.0 - blend_factor) + new_target_pos * blend_factor;
            
            camera_state.target = Point3::new(
                final_target.x,
                final_target.y,
                final_target.z
            );
        }
    }
}
