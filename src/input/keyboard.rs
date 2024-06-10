// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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