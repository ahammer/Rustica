//! Core event handling functionality

use std::marker::PhantomData;
use std::collections::VecDeque;
use rustica_ecs::entity::Entity;
use rustica_ecs::world::World;
use crate::error::Result;

/// Trait for event types in the Rustica engine
pub trait Event: Send + Sync + 'static {}

// Implement Event for common types that could be used as events
impl<T: Send + Sync + 'static> Event for T {}

/// Storage for events of a specific type
#[derive(Default)]
pub struct Events<T: Event> {
    /// Events that were added in the current frame
    current_events: Vec<T>,
    /// Events that were added in the previous frame and can be read this frame
    available_events: VecDeque<T>,
    /// Whether events have been updated this frame
    updated: bool,
}

impl<T: Event> Events<T> {
    /// Create a new Events container
    pub fn new() -> Self {
        Self {
            current_events: Vec::new(),
            available_events: VecDeque::new(),
            updated: false,
        }
    }

    /// Add an event to be processed this frame
    pub fn send(&mut self, event: T) {
        self.current_events.push(event);
    }

    /// Update the events, making current events available for reading
    /// and clearing old events. This should be called once per frame.
    pub fn update(&mut self) {
        self.available_events.clear();
        std::mem::swap(&mut self.current_events, &mut self.available_events.make_contiguous().to_vec());
        self.current_events.clear();
        self.updated = true;
    }

    /// Return an iterator over the events
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.available_events.iter()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.current_events.clear();
        self.available_events.clear();
    }
}

/// Reader for events of type T
pub struct EventReader<T: Event> {
    /// Current read index
    read_index: usize,
    /// Marker for type T
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventReader<T> {
    fn default() -> Self {
        Self {
            read_index: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Event> EventReader<T> {
    /// Create a new EventReader
    pub fn new() -> Self {
        Self::default()
    }

    /// Read events from the Events resource
    pub fn read(&mut self, events: &Events<T>) -> impl Iterator<Item = &T> {
        let slice = &events.available_events;
        let old_index = self.read_index;
        self.read_index = slice.len();
        
        slice.iter().skip(old_index)
    }
}

/// Writer for events of type T
pub struct EventWriter<'w, T: Event> {
    /// Reference to the Events resource
    events: &'w mut Events<T>,
}

impl<'w, T: Event> EventWriter<'w, T> {
    /// Create a new EventWriter
    pub fn new(events: &'w mut Events<T>) -> Self {
        Self { events }
    }

    /// Send an event
    pub fn send(&mut self, event: T) {
        self.events.send(event);
    }
}
