//! Event System implementation for Rustica Engine
//!
//! This module provides the EventSystem class that manages events
//! and is directly integrated with the App structure.

use std::any::{Any, TypeId};
use std::collections::HashMap;

use rustica_ecs::World;
use crate::event::{Event, Events};

/// EventSystem manages all events in the Rustica engine.
///
/// It provides methods for registering event types, sending events,
/// and processing events on each frame update.
pub struct EventSystem {
    /// Event queues for each event type
    event_queues: HashMap<TypeId, Box<dyn Any>>,
}

impl EventSystem {
    /// Create a new EventSystem instance.
    pub fn new() -> Self {
        Self {
            event_queues: HashMap::new(),
        }
    }

    /// Register an event type with the system.
    pub fn register_event<E: Event + Clone>(&mut self) {
        let type_id = TypeId::of::<E>();
        if !self.event_queues.contains_key(&type_id) {
            let events = Events::<E>::new();
            self.event_queues.insert(type_id, Box::new(events));
        }
    }

    /// Send an event to be processed.
    pub fn send_event<E: Event + Clone>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        
        // Register the event type if not already registered
        if !self.event_queues.contains_key(&type_id) {
            self.register_event::<E>();
        }
        
        // Get the events container and send the event
        if let Some(events) = self.event_queues.get_mut(&type_id) {
            if let Some(events) = events.downcast_mut::<Events<E>>() {
                events.send(event);
            }
        }
    }

    /// Process events for a given world.
    ///
    /// This should be called once per frame to update event queues
    /// and process events for the world.
    pub fn process_events(&mut self, _world: &mut World) {
        // Update all event queues
        for (_, events) in self.event_queues.iter_mut() {
            // Get the concrete type through type erasure and update
            if let Some(events) = events.downcast_mut::<Events<()>>() {
                events.update();
            } else {
                // This is a workaround since we can't easily update events
                // of unknown types. In a real implementation, we'd use a trait
                // with an update method to handle this properly.
            }
        }
        
        // In a more complete implementation, this would also dispatch
        // events to relevant systems or handlers in the world
    }

    /// Get a reference to an event queue.
    pub fn get_events<E: Event + Clone>(&self) -> Option<&Events<E>> {
        let type_id = TypeId::of::<E>();
        self.event_queues.get(&type_id)
            .and_then(|events| events.downcast_ref::<Events<E>>())
    }

    /// Get a mutable reference to an event queue.
    pub fn get_events_mut<E: Event + Clone>(&mut self) -> Option<&mut Events<E>> {
        let type_id = TypeId::of::<E>();
        self.event_queues.get_mut(&type_id)
            .and_then(|events| events.downcast_mut::<Events<E>>())
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}
