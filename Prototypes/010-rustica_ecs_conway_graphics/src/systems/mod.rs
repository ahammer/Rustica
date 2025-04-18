// Systems module - exports animation systems


pub mod animation_systems;
pub mod spawner_system; // Added spawner system module

// Export systems for use in main.rs
pub use animation_systems::{VisualAnimationSystem, CameraAnimationSystem};
pub use spawner_system::CellSpawnerSystem; // Added spawner system export
