//! # World Container
//! 
//! This module defines the World struct, which is the main container
//! for entities and components in the ECS system.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#World
//! - API_RULES: /docs/api_conventions.md#WorldAPI
//! 
//! ## Critical Rules
//! 
//! 1. The World should efficiently manage entities and components
//! 2. All component access must be type-safe
//! 3. Entity lifecycles must be properly managed

// === REGION: IMPORTS ===
use std::any::TypeId;
use std::collections::HashMap;

use crate::entity::{Entity, EntityBuilder};
use crate::component::{Component, ComponentStorage};
use crate::query::{Query, QueryResult};

// === REGION: WORLD DEFINITION ===

/// The central container for entities and components in the ECS system.
///
/// The World manages entity creation and destruction, component storage,
/// and provides access to components through queries.
///
/// # Examples
///
/// ```
/// use rustica_ecs::prelude::*;
///
/// // Create a new world
/// let mut world = World::new();
///
/// // Spawn an entity with components
/// let entity = world.spawn()
///     .insert(42u32)
///     .insert("Hello".to_string())
///     .id();
///
/// // Access components
/// assert_eq!(*world.get::<u32>(entity).unwrap(), 42);
/// assert_eq!(*world.get::<String>(entity).unwrap(), "Hello");
/// ```
pub struct World {
    // The next available entity ID
    next_entity_id: u64,
    
    // Component storages by type ID
    storages: HashMap<TypeId, ComponentStorage>,
}

impl World {
    /// Creates a new, empty world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::World;
    ///
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            storages: HashMap::new(),
        }
    }
    
    /// Spawns a new entity and returns an EntityBuilder for adding components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn().id();
    /// ```
    pub fn spawn(&mut self) -> EntityBuilder {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        
        EntityBuilder::new(self, entity)
    }
    
    /// Despawns an entity, removing all its components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn().id();
    /// world.despawn(entity);
    /// ```
    pub fn despawn(&mut self, _entity: Entity) {
        // In a real implementation, this would remove all components
        // For now, just a stub
    }
    
    /// Gets a reference to a component for an entity, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn()
    ///     .insert(42u32)
    ///     .id();
    ///
    /// if let Some(value) = world.get::<u32>(entity) {
    ///     assert_eq!(*value, 42);
    /// }
    /// ```
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        // In a real implementation, this would retrieve the component
        // For now, just a stub that always returns None
        None
    }
    
    /// Gets a mutable reference to a component for an entity, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn()
    ///     .insert(42u32)
    ///     .id();
    ///
    /// if let Some(value) = world.get_mut::<u32>(entity) {
    ///     *value = 84;
    /// }
    ///
    /// if let Some(value) = world.get::<u32>(entity) {
    ///     assert_eq!(*value, 84);
    /// }
    /// ```
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        // In a real implementation, this would retrieve the component
        // For now, just a stub that always returns None
        None
    }
    
    /// Creates a query for accessing components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// struct Position { x: f32, y: f32, z: f32 }
    /// struct Velocity { x: f32, y: f32, z: f32 }
    ///
    /// let mut world = World::new();
    /// world.spawn()
    ///     .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
    ///     .insert(Velocity { x: 1.0, y: 0.0, z: 0.0 });
    ///
    /// for (_position, _velocity) in world.query::<(&Position, &Velocity)>() {
    ///     // Do something with the components
    /// }
    /// ```
    pub fn query<Q: Query>(&self) -> QueryResult<Q> {
        // In a real implementation, this would create a query
        // For now, just a stub
        QueryResult::new()
    }
    
    /// Adds a component to an entity.
    ///
    /// This is primarily used by EntityBuilder.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn().id();
    /// world.insert(entity, 42u32);
    ///
    /// assert_eq!(*world.get::<u32>(entity).unwrap(), 42);
    /// ```
    pub fn insert<T: Component>(&mut self, _entity: Entity, _component: T) {
        // In a real implementation, this would add the component
        // For now, just a stub
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_world_creation() {
        let world = World::new();
        // Simply testing that the world can be created without errors
    }
    
    #[test]
    fn test_entity_spawn() {
        let mut world = World::new();
        let entity1 = world.spawn().id();
        let entity2 = world.spawn().id();
        
        // Entity IDs should be unique
        assert_ne!(entity1, entity2);
    }
}
