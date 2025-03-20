//! # Entity Management
//! 
//! This module defines the Entity type and related functionality.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#Entities
//! - API_RULES: /docs/api_conventions.md#EntityAPI
//! 
//! ## Critical Rules
//! 
//! 1. Entities are just identifiers, not containers for data
//! 2. Entity creation must be efficient
//! 3. Invalid entity references must be detectable

// === REGION: IMPORTS ===
use crate::World;
use crate::Component;

// === REGION: ENTITY DEFINITION ===

/// An entity identifier.
///
/// Entities are lightweight identifiers that represent an object in the game world.
/// They are not containers for data - data is stored in components.
///
/// # Thread Safety
///
/// Entities are `Send + Sync` and can be safely shared between threads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    /// Creates a new entity with the given ID.
    ///
    /// This is primarily for internal use. Users should create entities
    /// using `World::spawn()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::Entity;
    ///
    /// let entity = Entity::new(42);
    /// assert_eq!(entity.id(), 42);
    /// ```
    pub fn new(id: u64) -> Self {
        Entity(id)
    }
    
    /// Returns the entity's ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::Entity;
    ///
    /// let entity = Entity::new(42);
    /// assert_eq!(entity.id(), 42);
    /// ```
    pub fn id(&self) -> u64 {
        self.0
    }
}

// === REGION: ENTITY BUILDER ===

/// Builder for creating entities with components.
///
/// EntityBuilder is used to fluently add components to an entity
/// during creation.
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
/// let entity = world.spawn()
///     .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
///     .insert(Velocity { x: 1.0, y: 0.0, z: 0.0 })
///     .id();
/// ```
pub struct EntityBuilder<'a> {
    /// The world this builder belongs to
    world: &'a mut World,
    
    /// The entity being built
    entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    /// Creates a new EntityBuilder.
    ///
    /// This is primarily for internal use by `World::spawn()`.
    pub(crate) fn new(world: &'a mut World, entity: Entity) -> Self {
        EntityBuilder { world, entity }
    }
    
    /// Adds a component to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// struct Position { x: f32, y: f32, z: f32 }
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn()
    ///     .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
    ///     .id();
    /// ```
    pub fn insert<T: Component>(&mut self, component: T) -> &mut Self {
        self.world.insert(self.entity, component);
        self
    }
    
    /// Returns the entity ID being built.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn().id();
    /// ```
    pub fn id(&self) -> Entity {
        self.entity
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(42);
        assert_eq!(entity.id(), 42);
    }
    
    #[test]
    fn test_entity_equality() {
        let entity1 = Entity::new(42);
        let entity2 = Entity::new(42);
        let entity3 = Entity::new(43);
        
        assert_eq!(entity1, entity2);
        assert_ne!(entity1, entity3);
    }
}
