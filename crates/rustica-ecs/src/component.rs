// Component module - Contains component trait and related functionality

use std::any::Any;
use std::fmt::Debug;

/// Component trait - Implemented by all component types in the ECS
///
/// Components must be 'static (have a lifetime that lasts for the program's duration)
/// and implement Debug for easier debugging and inspection.
pub trait Component: 'static + Debug {}

/// Internal trait for type erasure in the component system
/// This is an implementation detail and not part of the public API
pub(crate) trait ComponentStorage {
    /// Convert this storage to Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Convert this storage to mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Implementation of component storage for a specific component type
impl<T: Component> ComponentStorage for std::collections::HashMap<crate::entity::Entity, T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
