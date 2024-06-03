use std::collections::HashMap;
use sfml::window::{Event, Key};

/// @brief Handles keyboard inputs and events.
pub struct Keyboard {
    keys: HashMap<Key, bool>,
    recently_released: Key
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            recently_released: Key::Unknown
        }
    }

    pub fn update(&mut self, e: Event) {
        match e {
            Event::KeyReleased { code, .. } => {
                self.keys.insert(code, false);
            }
            Event::KeyPressed { code, .. } => {
                self.recently_released = code;
                self.keys.insert(code, true);
            }
            _ => {}
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn key_released(&self, key: Key) -> bool {
        self.recently_released == key
    }
}