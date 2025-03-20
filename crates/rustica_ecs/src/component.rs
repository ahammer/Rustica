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
use std::any::TypeId;

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

/// Storage for components of a specific type.
///
/// This is an internal type used by the world to store components.
/// It is not part of the public API.
pub(crate) struct ComponentStorage {
    // todo: fix this - implement component storage
    // For now, it's just a stub
}

impl ComponentStorage {
    /// Creates a new ComponentStorage.
    pub(crate) fn new() -> Self {
        ComponentStorage {}
    }
    
    /// Gets a reference to a component for an entity, if it exists.
    pub(crate) fn get<T: Component>(&self, _entity_id: u64) -> Option<&T> {
        // todo: fix this - implement component retrieval
        // For now, just a stub that always returns None
        None
    }
    
    /// Gets a mutable reference to a component for an entity, if it exists.
    pub(crate) fn get_mut<T: Component>(&mut self, _entity_id: u64) -> Option<&mut T> {
        // todo: fix this - implement component mutable retrieval
        // For now, just a stub that always returns None
        None
    }
    
    /// Inserts a component for an entity.
    pub(crate) fn insert<T: Component>(&mut self, _entity_id: u64, _component: T) {
        // todo: fix this - implement component storage
        // For now, just a stub
    }
    
    /// Removes a component for an entity.
    pub(crate) fn remove<T: Component>(&mut self, _entity_id: u64) {
        // todo: fix this - implement component removal
        // For now, just a stub
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
