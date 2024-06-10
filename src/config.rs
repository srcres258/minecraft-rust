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

/// @brief Default configuration for program.
#[derive(Copy, Clone)]
pub struct Config {
    pub window_x: i32,
    pub window_y: i32,
    pub is_fullscreen: bool,
    pub render_distance: i32, // Set initial RD low to prevent long load times
    pub fov: i32
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_x: 1280,
            window_y: 720,
            is_fullscreen: false,
            render_distance: 8,
            fov: 90
        }
    }
}