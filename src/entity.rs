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

use crate::physics::aabb::AABB;

#[derive(Copy, Clone)]
pub struct Entity {
    pub position: glm::TVec3<f32>,
    pub rotation: glm::TVec3<f32>,
    pub velocity: glm::TVec3<f32>,

    pub box_aabb: AABB
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_ex_1(pos: &glm::TVec3<f32>, rot: &glm::TVec3<f32>) -> Self {
        Self {
            position: *pos,
            rotation: *rot,
            velocity: glm::vec3(0.0, 0.0, 0.0),
            box_aabb: AABB::new(&glm::vec3(0.0, 0.0, 0.0))
        }
    }

    pub fn new_ex_2(
        pos: &glm::TVec3<f32>,
        rot: &glm::TVec3<f32>,
        box_aabb: &glm::TVec3<f32>
    ) -> Self {
        Self {
            position: *pos,
            rotation: *rot,
            velocity: glm::vec3(0.0, 0.0, 0.0),
            box_aabb: AABB::new(box_aabb)
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            velocity: glm::vec3(0.0, 0.0, 0.0),
            box_aabb: AABB::new(&glm::vec3(0.0, 0.0, 0.0))
        }
    }
}