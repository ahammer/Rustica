//! Time tracking for ECS systems
//! 
//! This module provides functionality for tracking game time
//! and delta time between frames.

use std::time::Duration;

/// A simple timer resource for tracking delta time
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// The time elapsed since the last frame
    pub delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs_f32(1.0 / 60.0), // Default to 60 FPS
        }
    }
}

impl Time {
    /// Get the delta time in seconds
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    
    /// Update the delta time
    pub fn update(&mut self, delta: Duration) {
        self.delta = delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_time_delta_seconds() {
        let time = Time {
            delta: Duration::from_millis(16), // ~60 FPS
        };
        assert!((time.delta_seconds() - 0.016).abs() < 0.0001);
    }
    
    #[test]
    fn test_time_update() {
        let mut time = Time::default();
        let new_delta = Duration::from_millis(32); // ~30 FPS
        time.update(new_delta);
        assert_eq!(time.delta, new_delta);
    }
}
