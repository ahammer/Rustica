// Entity module - Contains entity type definition and entity builder implementation

/// Entity is just an ID in the ECS system
pub type Entity = usize;

use crate::world::World;
use crate::component::Component;

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
