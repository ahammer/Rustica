// Entity module - Contains entity type definition and entity builder implementation

/// Entity is just an ID in the ECS system
pub type Entity = usize;

use crate::world::World;
use crate::component::Component;

/// An extension trait to add useful methods to the Entity type
pub trait EntityExt {
    /// Creates an invalid entity reference (useful for initialization)
    fn invalid() -> Entity;
    
    /// Checks if this entity is a valid reference
    fn is_valid(&self) -> bool;
}

impl EntityExt for Entity {
    fn invalid() -> Entity {
        // Use usize::MAX as an invalid entity ID
        // This works because entities are created starting from 0 and incrementing
        usize::MAX
    }
    
    fn is_valid(&self) -> bool {
        *self != Entity::invalid()
    }
}

/// EntityBuilder - Helper for creating entities with components
pub struct EntityBuilder<'a> {
    pub(crate) entity: Entity,
    pub(crate) world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    /// Add a component to the entity being built
    pub fn with<T: Component>(self, component: T) -> Self {
        self.world.add_component(self.entity, component);
        self
    }
    
    /// Finalize entity creation and return the entity ID
    pub fn build(self) -> Entity {
        self.entity
    }
}
