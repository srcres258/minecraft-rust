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

extern crate nalgebra_glm as glm;

/// @brief Raycasting class associated with player's line of sight.
pub struct Ray {
    ray_start: glm::TVec3<f32>,
    ray_end: glm::TVec3<f32>,
    direction: glm::TVec3<f32>
}

impl Ray {
    pub fn new(position: &glm::TVec3<f32>, direction: &glm::TVec3<f32>) -> Self {
        Self {
            ray_start: *position,
            ray_end: *position,
            direction: *direction
        }
    }

    pub fn step(&mut self, scale: f32) {
        let yaw = (self.direction.y + 90.0).to_radians();
        let pitch = self.direction.x.to_radians();

        self.ray_end.x -= yaw.cos() * scale;
        self.ray_end.z -= yaw.sin() * scale;
        self.ray_end.y -= pitch.tan() * scale;
    }

    pub fn end(&self) -> glm::TVec3<f32> {
        self.ray_end
    }

    pub fn length(&self) -> f32 {
        glm::distance(&self.ray_start, &self.ray_end)
    }
}