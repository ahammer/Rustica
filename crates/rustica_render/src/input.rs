//! Input handling for the Rustica engine

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};

/// Input state that tracks which keys are currently pressed.
pub struct InputState {
    /// Set of currently pressed keys
    pressed_keys: HashSet<VirtualKeyCode>,
}

impl InputState {
    /// Create a new empty input state
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    /// Process a keyboard input event
    pub fn process_keyboard_input(&mut self, input: KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(key_code);
                },
                ElementState::Released => {
                    self.pressed_keys.remove(&key_code);
                },
            }
        }
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    /// Get a set of all currently pressed keys
    pub fn pressed_keys(&self) -> &HashSet<VirtualKeyCode> {
        &self.pressed_keys
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// A resource that holds the input state
#[derive(Clone)]
pub struct InputResource {
    /// The input state
    state: Arc<Mutex<InputState>>,
}

impl InputResource {
    /// Create a new input resource
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(InputState::new())),
        }
    }

    /// Process a keyboard input event
    pub fn process_keyboard_input(&self, input: KeyboardInput) {
        if let Ok(mut state) = self.state.lock() {
            state.process_keyboard_input(input);
        }
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        if let Ok(state) = self.state.lock() {
            state.is_key_pressed(key_code)
        } else {
            false
        }
    }

    /// Get a copy of the state
    pub fn state(&self) -> Arc<Mutex<InputState>> {
        self.state.clone()
    }
}

impl Default for InputResource {
    fn default() -> Self {
        Self::new()
    }
}

/// A system that handles input for camera movement
pub fn camera_input_system(app: &mut rustica_core::App) {
    // Get input and camera resources
    let input = match app.get_resource::<InputResource>() {
        Some(input) => input.clone(),
        None => return,
    };

    // Example of how the input system would work with a Camera resource
    // This would need to be implemented based on the actual Camera implementation
    // For now, just print the pressed keys for demonstration
    if let Ok(state) = input.state().lock() {
        let keys = state.pressed_keys();
        if !keys.is_empty() {
            println!("Pressed keys: {:?}", keys);
            
            // Example camera movement logic (would actually modify a camera resource)
            if keys.contains(&VirtualKeyCode::W) {
                println!("Moving camera forward");
            }
            if keys.contains(&VirtualKeyCode::S) {
                println!("Moving camera backward");
            }
            if keys.contains(&VirtualKeyCode::A) {
                println!("Moving camera left");
            }
            if keys.contains(&VirtualKeyCode::D) {
                println!("Moving camera right");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state() {
        let mut state = InputState::new();
        assert!(!state.is_key_pressed(VirtualKeyCode::Space));

        // Press a key
        let input = KeyboardInput {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            modifiers: Default::default(),
        };
        state.process_keyboard_input(input);
        assert!(state.is_key_pressed(VirtualKeyCode::Space));

        // Release the key
        let input = KeyboardInput {
            scancode: 0,
            state: ElementState::Released,
            virtual_keycode: Some(VirtualKeyCode::Space),
            modifiers: Default::default(),
        };
        state.process_keyboard_input(input);
        assert!(!state.is_key_pressed(VirtualKeyCode::Space));
    }
}
