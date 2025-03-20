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
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

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
    /// The next available entity ID
    next_entity_id: u64,
    
    /// Component storages by type ID
    storages: HashMap<TypeId, ComponentStorage>,
    
    /// Set of active entity IDs
    entities: HashSet<u64>,
    
    /// Resources that can be accessed globally
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
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
            entities: HashSet::new(),
            resources: HashMap::new(),
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
        
        self.entities.insert(entity.id());
        
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
    pub fn despawn(&mut self, entity: Entity) {
        let entity_id = entity.id();
        
        // Remove the entity from the active set
        self.entities.remove(&entity_id);
        
        // Remove all components for this entity
        for storage in self.storages.values_mut() {
            storage.remove::<()>(entity_id);
        }
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
        let type_id = TypeId::of::<T>();
        self.storages.get(&type_id)
            .and_then(|storage| storage.get::<T>(entity.id()))
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
        let type_id = TypeId::of::<T>();
        self.storages.get_mut(&type_id)
            .and_then(|storage| storage.get_mut::<T>(entity.id()))
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
        // For now, this is still a stub, as proper query implementation
        // requires more complex type handling. A future task would enhance
        // the query system with actual iteration over entities.
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
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        
        // Get or create the storage for this component type
        let storage = self.storages.entry(type_id)
            .or_insert_with(|| ComponentStorage::new::<T>());
        
        // Add the component to the storage
        storage.insert(entity.id(), component);
    }
    
    /// Returns the number of entities in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// assert_eq!(world.entity_count(), 0);
    ///
    /// world.spawn();
    /// assert_eq!(world.entity_count(), 1);
    /// ```
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    
    /// Returns whether the entity exists in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// let mut world = World::new();
    /// let entity = world.spawn().id();
    ///
    /// assert!(world.contains(entity));
    ///
    /// world.despawn(entity);
    /// assert!(!world.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(&entity.id())
    }
    
    /// Inserts a resource into the world.
    ///
    /// Resources are global data that can be accessed by systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// #[derive(Debug)]
    /// struct Time { delta: f32 }
    ///
    /// let mut world = World::new();
    /// world.insert_resource(Time { delta: 0.016 });
    ///
    /// let time = world.get_resource::<Time>().unwrap();
    /// assert_eq!(time.delta, 0.016);
    /// ```
    pub fn insert_resource<T: 'static + Send + Sync>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(resource));
    }
    
    /// Gets a reference to a resource, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// struct Time { delta: f32 }
    ///
    /// let mut world = World::new();
    /// world.insert_resource(Time { delta: 0.016 });
    ///
    /// if let Some(time) = world.get_resource::<Time>() {
    ///     assert_eq!(time.delta, 0.016);
    /// }
    /// ```
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.resources.get(&type_id)
            .and_then(|resource| resource.downcast_ref::<T>())
    }
    
    /// Gets a mutable reference to a resource, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustica_ecs::prelude::*;
    ///
    /// struct Time { delta: f32 }
    ///
    /// let mut world = World::new();
    /// world.insert_resource(Time { delta: 0.016 });
    ///
    /// if let Some(time) = world.get_resource_mut::<Time>() {
    ///     time.delta = 0.033;
    /// }
    ///
    /// let time = world.get_resource::<Time>().unwrap();
    /// assert_eq!(time.delta, 0.033);
    /// ```
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.resources.get_mut(&type_id)
            .and_then(|resource| resource.downcast_mut::<T>())
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
        assert_eq!(world.entity_count(), 0);
    }
    
    #[test]
    fn test_entity_spawn() {
        let mut world = World::new();
        let entity1 = world.spawn().id();
        let entity2 = world.spawn().id();
        
        // Entity IDs should be unique
        assert_ne!(entity1, entity2);
        
        // Two entities should exist in the world
        assert_eq!(world.entity_count(), 2);
    }
    
    #[test]
    fn test_entity_despawn() {
        let mut world = World::new();
        let entity = world.spawn().id();
        
        assert!(world.contains(entity));
        
        world.despawn(entity);
        
        assert!(!world.contains(entity));
        assert_eq!(world.entity_count(), 0);
    }
    
    #[test]
    fn test_component_insert_get() {
        let mut world = World::new();
        let entity = world.spawn().id();
        
        world.insert(entity, 42u32);
        
        assert_eq!(*world.get::<u32>(entity).unwrap(), 42);
    }
    
    #[test]
    fn test_component_update() {
        let mut world = World::new();
        let entity = world.spawn().id();
        
        world.insert(entity, 42u32);
        
        {
            let value = world.get_mut::<u32>(entity).unwrap();
            *value = 84;
        }
        
        assert_eq!(*world.get::<u32>(entity).unwrap(), 84);
    }
    
    #[test]
    fn test_entity_with_multiple_components() {
        let mut world = World::new();
        let entity = world.spawn().id();
        
        world.insert(entity, 42u32);
        world.insert(entity, "hello".to_string());
        
        assert_eq!(*world.get::<u32>(entity).unwrap(), 42);
        assert_eq!(*world.get::<String>(entity).unwrap(), "hello");
    }
    
    #[test]
    fn test_resource_insert_get() {
        struct TestResource {
            value: i32,
        }
        
        let mut world = World::new();
        world.insert_resource(TestResource { value: 42 });
        
        let resource = world.get_resource::<TestResource>().unwrap();
        assert_eq!(resource.value, 42);
    }
    
    #[test]
    fn test_resource_update() {
        struct TestResource {
            value: i32,
        }
        
        let mut world = World::new();
        world.insert_resource(TestResource { value: 42 });
        
        {
            let resource = world.get_resource_mut::<TestResource>().unwrap();
            resource.value = 84;
        }
        
        let resource = world.get_resource::<TestResource>().unwrap();
        assert_eq!(resource.value, 84);
    }
}
