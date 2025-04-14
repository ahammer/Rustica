// System module - Contains the System trait definition

use crate::world::World;

/// System trait - Implemented by all systems in the ECS
///
/// Systems are responsible for processing entities and their components
/// according to game logic rules.
pub trait System {
    /// Run the system on the given world
    ///
    /// This is where the system's logic is implemented, typically querying
    /// for entities with specific components and processing them.
    fn run(&self, world: &mut World);
}
