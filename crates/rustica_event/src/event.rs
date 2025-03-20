//! Core event handling functionality

use std::marker::PhantomData;
use std::collections::VecDeque;

/// Trait for event types in the Rustica engine
pub trait Event: Send + Sync + 'static {}

// Implement Event for common types that could be used as events
impl<T: Send + Sync + 'static> Event for T {}

/// Storage for events of a specific type
#[derive(Default)]
pub struct Events<T: Event + Clone> {
    /// Events that were added in the current frame
    current_events: Vec<T>,
    /// Events that were added in the previous frame and can be read this frame
    available_events: VecDeque<T>,
    /// Incremented each time update() is called to track event generations
    update_counter: usize,
}

impl<T: Event + Clone> Events<T> {
    /// Create a new Events container
    pub fn new() -> Self {
        Self {
            current_events: Vec::new(),
            available_events: VecDeque::new(),
            update_counter: 0,
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
        
        // Move current events to available_events
        for event in self.current_events.drain(..) {
            self.available_events.push_back(event);
        }
        
        // Increment the update counter to mark a new generation of events
        self.update_counter += 1;
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
    /// Last update counter we've seen from the events container
    last_update_counter: usize,
    /// Last known event count we've processed
    last_event_count: usize,
    /// Marker for type T
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventReader<T> {
    fn default() -> Self {
        Self {
            last_update_counter: 0,
            last_event_count: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Event + Clone> EventReader<T> {
    /// Create a new EventReader
    pub fn new() -> Self {
        Self::default()
    }

    /// Read events from the Events resource
    pub fn read<'a>(&mut self, events: &'a Events<T>) -> impl Iterator<Item = &'a T> + 'a {
        // Check if events container has been updated since our last read
        if self.last_update_counter != events.update_counter {
            // New update detected, reset our event count
            self.last_update_counter = events.update_counter;
            self.last_event_count = 0;
        }
        
        // Remember how many events we've seen
        let old_count = self.last_event_count;
        self.last_event_count = events.available_events.len();
        
        // Return only newly added events
        events.available_events.iter().skip(old_count)
    }
}

/// Writer for events of type T
pub struct EventWriter<'w, T: Event + Clone> {
    /// Reference to the Events resource
    events: &'w mut Events<T>,
}

impl<'w, T: Event + Clone> EventWriter<'w, T> {
    /// Create a new EventWriter
    pub fn new(events: &'w mut Events<T>) -> Self {
        Self { events }
    }

    /// Send an event
    pub fn send(&mut self, event: T) {
        self.events.send(event);
    }
}
