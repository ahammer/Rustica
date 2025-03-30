// World module - Contains the main ECS container and related operations

use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use crate::component::{Component, ComponentStorage};
use crate::entity::{Entity, EntityBuilder};
use crate::system::System;

/// World - The main container for the ECS
///
/// World manages entities, components, and systems, providing methods
/// for creating and querying entities with components and running systems.
pub struct World {
    // Entity management
    next_entity_id: Entity,
    entities: HashSet<Entity>,
    
    // Component storage: Maps Component type to a map of Entity -> Component instance
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
    
    // Systems
    systems: Vec<Box<dyn System>>,
}

impl World {
    /// Create a new empty World
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            entities: HashSet::new(),
            components: HashMap::new(),
            systems: Vec::new(),
        }
    }
    
    /// Register a component type
    ///
    /// This pre-allocates storage for a component type, but is not strictly
    /// necessary as components will be automatically registered when first used.
    pub fn register<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            let component_store: HashMap<Entity, T> = HashMap::new();
            self.components.insert(type_id, Box::new(component_store));
        }
    }
    
    /// Create a new entity
    ///
    /// Returns an EntityBuilder which can be used to add components to the entity.
    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.insert(entity);
        
        EntityBuilder {
            entity,
            world: self,
        }
    }
    
    /// Add a component to an entity
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        
        if let Some(component_store) = self.components.get_mut(&type_id) {
            if let Some(store) = component_store.as_any_mut().downcast_mut::<HashMap<Entity, T>>() {
                store.insert(entity, component);
            }
        } else {
            // Register the component type if it doesn't exist
            let mut component_store: HashMap<Entity, T> = HashMap::new();
            component_store.insert(entity, component);
            self.components.insert(type_id, Box::new(component_store));
        }
    }
    
    /// Get a component for an entity
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        
        self.components
            .get(&type_id)
            .and_then(|component_store| {
                component_store
                    .as_any()
                    .downcast_ref::<HashMap<Entity, T>>()
                    .and_then(|store| store.get(&entity))
            })
    }
    
    /// Get mutable component for an entity
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        
        self.components
            .get_mut(&type_id)
            .and_then(|component_store| {
                component_store
                    .as_any_mut()
                    .downcast_mut::<HashMap<Entity, T>>()
                    .and_then(|store| store.get_mut(&entity))
            })
    }
    
    /// Query for entities with a specific component
    ///
    /// Returns a vector of (Entity, &Component) pairs for all entities that have
    /// the specified component type.
    pub fn query_one<T: Component>(&self) -> Vec<(Entity, &T)> {
        let type_id = TypeId::of::<T>();
        
        if let Some(component_store) = self.components.get(&type_id) {
            if let Some(store) = component_store.as_any().downcast_ref::<HashMap<Entity, T>>() {
                return store
                    .iter()
                    .map(|(&entity, component)| (entity, component))
                    .collect();
            }
        }
        
        Vec::new()
    }
    
    /// Query for entities with two components
    ///
    /// Returns a vector of (Entity, (&ComponentA, &ComponentB)) tuples for all entities
    /// that have both component types.
    pub fn query_two<A: Component, B: Component>(&self) -> Vec<(Entity, (&A, &B))> {
        let mut result = Vec::new();
        
        // Get stores for both component types
        let a_type_id = TypeId::of::<A>();
        let b_type_id = TypeId::of::<B>();
        
        let a_store = match self.components.get(&a_type_id) {
            Some(store) => match store.as_any().downcast_ref::<HashMap<Entity, A>>() {
                Some(store) => store,
                None => return Vec::new(),
            },
            None => return Vec::new(),
        };
        
        let b_store = match self.components.get(&b_type_id) {
            Some(store) => match store.as_any().downcast_ref::<HashMap<Entity, B>>() {
                Some(store) => store,
                None => return Vec::new(),
            },
            None => return Vec::new(),
        };
        
        // Find entities that have both components
        for (&entity, a_comp) in a_store.iter() {
            if let Some(b_comp) = b_store.get(&entity) {
                result.push((entity, (a_comp, b_comp)));
            }
        }
        
        result
    }
    
    /// Add a system to the world
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
    
    /// Run all systems
    pub fn run_systems(&mut self) {
        // Take ownership of systems to avoid borrow checker issues
        let systems = std::mem::take(&mut self.systems);
        
        for system in &systems {
            system.run(self);
        }
        
        // Restore systems
        self.systems = systems;
    }
}
