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

use sfml::SfBox;
use sfml::system::Clock;
use sfml::window::Key;

/// @brief A keyboard related subclass that determines if a key remains pressed.
pub struct ToggleKey {
    key: Key,
    delay_timer: SfBox<Clock>
}

impl ToggleKey {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            delay_timer: Clock::start()
        }
    }

    pub fn is_key_pressed(&mut self) -> bool {
        if self.delay_timer.elapsed_time().as_seconds() > 0.2 {
            if self.key.is_pressed() {
                self.delay_timer.restart();
                return true;
            }
        }
        false
    }
}