//! # Component System
//! 
//! This module defines the Component trait and related functionality.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#Components
//! - API_RULES: /docs/api_conventions.md#ComponentAPI
//! 
//! ## Critical Rules
//! 
//! 1. Components must be plain data without behavior
//! 2. Components must be Send + Sync for thread safety
//! 3. Component storage should be optimized for cache locality

// === REGION: IMPORTS ===
// use std::any::TypeId; - removed unused import

// === REGION: COMPONENT TRAIT ===

/// A component is a piece of data that can be attached to an entity.
///
/// Components are plain data structures without behavior. They are combined
/// with entities to create game objects with different capabilities.
///
/// # Thread Safety
///
/// Components must be `Send + Sync` to support parallel processing.
///
/// # Examples
///
/// ```
/// use rustica_ecs::prelude::*;
///
/// // A position component
/// #[derive(Debug)]
/// struct Position {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// // A velocity component
/// #[derive(Debug)]
/// struct Velocity {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// // These types are automatically components because they are 'static + Send + Sync
/// ```
pub trait Component: 'static + Send + Sync {}

// Blanket implementation for all eligible types
impl<T: 'static + Send + Sync> Component for T {}

// === REGION: COMPONENT STORAGE ===

use std::collections::HashMap;
use std::any::Any;

/// Storage for components of a specific type.
///
/// This is an internal type used by the world to store components.
/// It is not part of the public API.
///
/// The ComponentStorage uses a combination of:
/// - A HashMap for entity ID to component index mapping (sparse set pattern)
/// - A Vec for contiguous storage of components (dense array)
/// - A Vec for mapping component indices back to entity IDs
///
/// This design optimizes for cache locality during iteration while maintaining
/// fast entity-based lookups.
pub(crate) struct ComponentStorage {
    /// The actual components, stored in a type-erased manner
    components: Box<dyn ComponentVec>,
    /// Maps entity IDs to indices in the components vec
    entity_to_index: HashMap<u64, usize>,
    /// Maps indices back to entity IDs for iteration
    index_to_entity: Vec<u64>,
}

/// A trait for type-erased component vectors
trait ComponentVec: Any + Send + Sync {
    /// Get a reference to a component by index
    fn get(&self, index: usize) -> Option<&dyn Any>;
    /// Get a mutable reference to a component by index
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Any>;
    /// Remove a component by index, swapping with the last element
    fn swap_remove(&mut self, index: usize);
    /// Insert a component, returning its index
    fn push(&mut self, component: Box<dyn Any>) -> usize;
    /// Get the number of components
    fn len(&self) -> usize;
}

/// A concrete implementation of ComponentVec for a specific component type
struct TypedComponentVec<T: Component> {
    /// The components of type T
    components: Vec<T>,
}

impl<T: Component> TypedComponentVec<T> {
    /// Create a new empty TypedComponentVec
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}

impl<T: Component> ComponentVec for TypedComponentVec<T> {
    fn get(&self, index: usize) -> Option<&dyn Any> {
        self.components.get(index).map(|component| component as &dyn Any)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Any> {
        self.components.get_mut(index).map(|component| component as &mut dyn Any)
    }

    fn swap_remove(&mut self, index: usize) {
        if index < self.components.len() {
            self.components.swap_remove(index);
        }
    }

    fn push(&mut self, component: Box<dyn Any>) -> usize {
        let component = component.downcast::<T>()
            .expect("Component type mismatch in push");
        self.components.push(*component);
        self.components.len() - 1
    }

    fn len(&self) -> usize {
        self.components.len()
    }
}

impl ComponentStorage {
    /// Creates a new ComponentStorage for a specific component type.
    pub(crate) fn new<T: Component>() -> Self {
        ComponentStorage {
            components: Box::new(TypedComponentVec::<T>::new()),
            entity_to_index: HashMap::new(),
            index_to_entity: Vec::new(),
        }
    }
    
    /// Gets a reference to a component for an entity, if it exists.
    pub(crate) fn get<T: Component>(&self, entity_id: u64) -> Option<&T> {
        self.entity_to_index.get(&entity_id)
            .and_then(|&index| self.components.get(index))
            .and_then(|any_component| any_component.downcast_ref::<T>())
    }
    
    /// Gets a mutable reference to a component for an entity, if it exists.
    pub(crate) fn get_mut<T: Component>(&mut self, entity_id: u64) -> Option<&mut T> {
        if let Some(&index) = self.entity_to_index.get(&entity_id) {
            if let Some(any_component) = self.components.get_mut(index) {
                return any_component.downcast_mut::<T>();
            }
        }
        None
    }
    
    /// Inserts a component for an entity.
    pub(crate) fn insert<T: Component>(&mut self, entity_id: u64, component: T) {
        // If the entity already has a component of this type, update it
        if let Some(&index) = self.entity_to_index.get(&entity_id) {
            if let Some(any_component) = self.components.get_mut(index) {
                if let Some(typed_component) = any_component.downcast_mut::<T>() {
                    *typed_component = component;
                    return;
                }
            }
        }
        
        // Otherwise, add a new component
        let boxed_component: Box<dyn Any> = Box::new(component);
        let index = self.components.push(boxed_component);
        self.entity_to_index.insert(entity_id, index);
        
        // If we're adding at the end, extend the index_to_entity vec
        if index == self.index_to_entity.len() {
            self.index_to_entity.push(entity_id);
        } else {
            // Otherwise, update the existing entry
            self.index_to_entity[index] = entity_id;
        }
    }
    
    /// Removes a component for an entity.
    pub(crate) fn remove<T: Component>(&mut self, entity_id: u64) {
        if let Some(index) = self.entity_to_index.remove(&entity_id) {
            // Get the ID of the entity that will be moved to this index after swap_remove
            let last_index = self.components.len() - 1;
            if index < last_index {
                let moved_entity_id = self.index_to_entity[last_index];
                self.index_to_entity[index] = moved_entity_id;
                self.entity_to_index.insert(moved_entity_id, index);
            }
            
            // Remove the component and update the index_to_entity vec
            self.components.swap_remove(index);
            if last_index > 0 {
                self.index_to_entity.pop();
            }
        }
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test that various types implement Component
    #[test]
    fn test_component_implementations() {
        struct TestComponent1(i32);
        struct TestComponent2 { value: String }
        struct TestComponent3;
        
        // These tests just verify that the types implement Component
        // by calling a function that takes a Component trait object
        fn assert_component<T: Component>(_: T) {}
        
        assert_component(TestComponent1(42));
        assert_component(TestComponent2 { value: "test".to_string() });
        assert_component(TestComponent3);
    }
}
