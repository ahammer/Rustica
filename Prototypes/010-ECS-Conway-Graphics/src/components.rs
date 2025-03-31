// Visual and camera components for Conway's Game of Life

use rustica_ecs::prelude::*;
use cgmath::{Point3, Vector3};

/// Visual component - Represents the visual appearance of a cell
#[derive(Debug, Clone)]
pub struct CellVisual {
    // Current visual properties
    pub scale: f32,           // Current scale factor (0.0-1.0)
    pub color: [f32; 3],      // Current RGB color 
    
    // Animation targets and state
    pub target_scale: f32,    // Target scale to animate toward
    pub target_color: [f32; 3], // Target color to animate toward
    pub transition_time: f32, // How long the transition has been running (in seconds)
    pub is_transitioning: bool, // Whether this cell is currently animating
}

impl Component for CellVisual {}

impl Default for CellVisual {
    fn default() -> Self {
        Self {
            scale: 0.2,          // Start small
            color: [0.5, 0.5, 0.5], // Start grey
            target_scale: 0.2,    
            target_color: [0.5, 0.5, 0.5],
            transition_time: 0.0,
            is_transitioning: false,
        }
    }
}

/// Camera state component - Represents the animated camera state
#[derive(Debug, Clone)]
pub struct CameraState {
    // Current position and target
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    
    // Target position and look target (to animate toward)
    pub target_position: Point3<f32>,
    pub target_look_at: Point3<f32>,
    
    // Animation state
    pub transition_time: f32,
    pub is_transitioning: bool,
    
    // Parameters
    pub height_offset: f32,      // How high above the grid to position
    pub distance_multiplier: f32, // How far back to position based on grid size
    
    // Spring physics parameters for camera position
    pub spring_stiffness: f32,   // How quickly the camera responds (higher = faster)
    pub spring_damping: f32,     // Damping to prevent oscillation (higher = less oscillation)
    pub velocity: Vector3<f32>,  // Current velocity of the camera
    
    // Spring physics parameters for camera target (look-at point)
    pub target_spring_stiffness: f32, // How quickly the camera target responds
    pub target_spring_damping: f32,   // Damping for target movement
    pub target_velocity: Vector3<f32>, // Velocity of the target point
    
    // Orbit parameters
    pub orbit_angle: f32,        // Current angle of orbit in radians
    pub orbit_speed: f32,        // Speed of orbit in radians per second
    pub orbit_radius: f32,       // Radius of orbit path
    pub orbit_height: f32,       // Height of camera during orbit
}

impl Component for CameraState {}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 10.0, 10.0),
            target: Point3::new(0.0, 0.0, 0.0),
            target_position: Point3::new(0.0, 10.0, 10.0),
            target_look_at: Point3::new(0.0, 0.0, 0.0),
            transition_time: 0.0,
            is_transitioning: false,
            height_offset: 5.0,
            distance_multiplier: 0.75,
            
            // Camera position spring parameters - faster response for position
            spring_stiffness: 2.8,     // Moderate stiffness for camera position
            spring_damping: 1.2,       // Moderate damping to reduce oscillation
            velocity: Vector3::new(0.0, 0.0, 0.0),
            
            // Camera target spring parameters - slower, smoother for target
            target_spring_stiffness: 1.2,  // Lower stiffness for much smoother target movement
            target_spring_damping: 1.8,    // Higher damping for target to avoid overshooting
            target_velocity: Vector3::new(0.0, 0.0, 0.0),
            
            // Orbit parameters
            orbit_angle: 0.0,
            orbit_speed: 0.15,         // Slower orbit for smoother appearance
            orbit_radius: 20.0,        // Larger radius to see more of the board
            orbit_height: 12.0,        // Higher position for better overview
        }
    }
}

/// Cell instance data for rendering with instancing
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CellInstance {
    pub model_matrix: [[f32; 4]; 4],
    pub color: [f32; 3],
    pub _padding: u32, // For memory alignment
}

impl CellInstance {
    pub fn new(model_matrix: [[f32; 4]; 4], color: [f32; 3]) -> Self {
        Self {
            model_matrix,
            color,
            _padding: 0,
        }
    }
}
