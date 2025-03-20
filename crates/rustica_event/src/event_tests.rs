use crate::event::{Events, EventReader, EventWriter};

// Define a simple test event
#[derive(Clone, Debug, PartialEq)]
struct TestEvent {
    value: i32,
}
// The Event trait is automatically implemented for any type that is Send + Sync + 'static

#[cfg(test)]
mod events_tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let mut events = Events::<TestEvent>::new();
        assert_eq!(events.iter().count(), 0, "New events container should be empty");
    }

    #[test]
    fn test_event_send() {
        let mut events = Events::<TestEvent>::new();
        events.send(TestEvent { value: 42 });
        
        // Before update, the event shouldn't be available for reading
        assert_eq!(events.iter().count(), 0, "Events should not be readable before update");
        
        // After update, the event should be available
        events.update();
        assert_eq!(events.iter().count(), 1, "Events should be readable after update");
        assert_eq!(events.iter().next().unwrap().value, 42, "Event value should match");
    }

    #[test]
    fn test_multiple_events() {
        let mut events = Events::<TestEvent>::new();
        events.send(TestEvent { value: 1 });
        events.send(TestEvent { value: 2 });
        events.send(TestEvent { value: 3 });
        
        events.update();
        
        let values: Vec<i32> = events.iter().map(|e| e.value).collect();
        assert_eq!(values, vec![1, 2, 3], "Events should be read in order sent");
    }

    #[test]
    fn test_event_clear() {
        let mut events = Events::<TestEvent>::new();
        events.send(TestEvent { value: 1 });
        events.update();
        
        assert_eq!(events.iter().count(), 1, "Event should be readable after update");
        
        events.clear();
        assert_eq!(events.iter().count(), 0, "Events should be empty after clear");
    }

    #[test]
    fn test_event_update_lifecycle() {
        let mut events = Events::<TestEvent>::new();
        
        // Send events in frame 1
        events.send(TestEvent { value: 1 });
        events.send(TestEvent { value: 2 });
        
        // Update to make frame 1 events available
        events.update();
        assert_eq!(events.iter().count(), 2, "Should have 2 events from frame 1");
        
        // Send events in frame 2
        events.send(TestEvent { value: 3 });
        events.send(TestEvent { value: 4 });
        
        // Events from frame 1 should still be available, frame 2 not yet
        assert_eq!(events.iter().count(), 2, "Should still have only frame 1 events");
        
        // Update to make frame 2 events available and clear frame 1 events
        events.update();
        assert_eq!(events.iter().count(), 2, "Should have 2 events from frame 2");
        
        let values: Vec<i32> = events.iter().map(|e| e.value).collect();
        assert_eq!(values, vec![3, 4], "Should have frame 2 events only");
    }
}

#[cfg(test)]
mod event_reader_tests {
    use super::*;

    #[test]
    fn test_event_reader_creation() {
        let mut reader = EventReader::<TestEvent>::new();
        let mut events = Events::<TestEvent>::new();
        
        // New reader should read no events
        assert_eq!(reader.read(&events).count(), 0, "New reader should read no events");
    }

    #[test]
    fn test_event_reader_read() {
        let mut reader = EventReader::<TestEvent>::new();
        let mut events = Events::<TestEvent>::new();
        
        events.send(TestEvent { value: 1 });
        events.update();
        
        // Reader should read all events
        let read_events: Vec<i32> = reader.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, vec![1], "Reader should read all available events");
    }

    #[test]
    fn test_event_reader_read_multiple() {
        let mut reader = EventReader::<TestEvent>::new();
        let mut events = Events::<TestEvent>::new();
        
        events.send(TestEvent { value: 1 });
        events.update();
        
        // First read
        let read_events: Vec<i32> = reader.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, vec![1], "First read should get first event");
        
        // Second read should get nothing (already read)
        let read_events: Vec<i32> = reader.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, Vec::<i32>::new(), "Second read should get no events");
        
        // Add more events
        events.send(TestEvent { value: 2 });
        events.update();
        
        // Third read should get only new events
        let read_events: Vec<i32> = reader.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, vec![2], "Third read should get only new events");
    }

    #[test]
    fn test_multiple_readers() {
        let mut reader1 = EventReader::<TestEvent>::new();
        let mut reader2 = EventReader::<TestEvent>::new();
        let mut events = Events::<TestEvent>::new();
        
        events.send(TestEvent { value: 1 });
        events.update();
        
        // First reader reads
        let read_events: Vec<i32> = reader1.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, vec![1], "First reader should get the event");
        
        // Second reader should still get events independently
        let read_events: Vec<i32> = reader2.read(&events).map(|e| e.value).collect();
        assert_eq!(read_events, vec![1], "Second reader should get events independently");
    }
}

#[cfg(test)]
mod event_writer_tests {
    use super::*;

    #[test]
    fn test_event_writer_send() {
        let mut events = Events::<TestEvent>::new();
        let mut writer = EventWriter::new(&mut events);
        
        writer.send(TestEvent { value: 42 });
        events.update();
        
        assert_eq!(events.iter().count(), 1, "Writer should send events");
        assert_eq!(events.iter().next().unwrap().value, 42, "Writer should send correct event");
    }

    #[test]
    fn test_multiple_writers() {
        let mut events = Events::<TestEvent>::new();
        
        {
            let mut writer1 = EventWriter::new(&mut events);
            writer1.send(TestEvent { value: 1 });
        }
        
        {
            let mut writer2 = EventWriter::new(&mut events);
            writer2.send(TestEvent { value: 2 });
        }
        
        events.update();
        
        let values: Vec<i32> = events.iter().map(|e| e.value).collect();
        assert_eq!(values, vec![1, 2], "Multiple writers should work correctly");
    }
}
