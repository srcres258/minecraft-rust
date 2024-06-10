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

/// @brief Collision detection class for 3D environment.
#[derive(Copy, Clone, Default)]
pub struct AABB {
    pub position: glm::TVec3<f32>,
    pub dimensions: glm::TVec3<f32>
}

impl AABB {
    pub fn new(dim: &glm::TVec3<f32>) -> Self {
        Self {
            position: Default::default(),
            dimensions: *dim
        }
    }

    pub fn update(&mut self, location: &glm::TVec3<f32>) {
        self.position = *location;
    }

    pub fn get_vn(&self, normal: glm::TVec3<f32>) -> glm::TVec3<f32> {
        let mut res = self.position;

        if normal.x < 0.0 {
            res.x += self.dimensions.x;
        }
        if normal.y < 0.0 {
            res.y += self.dimensions.y;
        }
        if normal.z < 0.0 {
            res.z += self.dimensions.z;
        }

        res
    }

    pub fn get_vp(&self, normal: glm::TVec3<f32>) -> glm::TVec3<f32> {
        let mut res = self.position;

        if normal.x > 0.0 {
            res.x += self.dimensions.x;
        }
        if normal.y > 0.0 {
            res.y += self.dimensions.y;
        }
        if normal.z > 0.0 {
            res.z += self.dimensions.z;
        }

        res
    }
}