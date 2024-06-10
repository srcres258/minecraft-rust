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

use std::ops::{Deref, DerefMut};
use crate::config::Config;
use crate::entity::Entity;
use crate::maths::frustum::ViewFrustum;
use crate::maths::matrix;

pub struct PtrConstEntity(*const Entity);

pub struct Camera {
    pub base: Entity,
    
    p_entity: Option<PtrConstEntity>,

    frustum: ViewFrustum,

    projection_matrix: glm::TMat4<f32>,
    view_matrix: glm::TMat4<f32>,
    proj_view_matrix: glm::TMat4<f32>,

    #[allow(dead_code)]
    config: Config
}

impl Deref for PtrConstEntity {
    type Target = *const Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PtrConstEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Send for PtrConstEntity {}

impl Camera {
    pub fn new(config: Config) -> Self {
        let mut obj = Entity::default();
        obj.position = glm::vec3(0., 0., -3.5);
        let projection_matrix = matrix::make_projection_matrix(&config);

        Self {
            base: obj,
            p_entity: None,
            frustum: ViewFrustum::default(),
            projection_matrix,
            view_matrix: Default::default(),
            proj_view_matrix: Default::default(),
            config
        }
    }

    pub fn update(&mut self) {
        let wrapped_obj = &mut self.base;
        let p_entity = unsafe { &***self.p_entity.as_ref().unwrap() };
        wrapped_obj.position = glm::vec3(
            p_entity.position.x, p_entity.position.y + 0.6, p_entity.position.z
        );
        wrapped_obj.rotation = p_entity.rotation;

        self.view_matrix = matrix::make_view_matrix(self);
        self.proj_view_matrix = self.projection_matrix * self.view_matrix;
        self.frustum.update(&self.proj_view_matrix);

        // println!("Camera frustum: {:#?}", self.frustum);
    }

    pub fn hook_entity(&mut self, entity: *const Entity) {
        self.p_entity = Some(PtrConstEntity(entity));
    }

    pub fn get_view_matrix(&self) -> glm::TMat4<f32> {
        self.view_matrix
    }

    pub fn get_proj_matrix(&self) -> glm::TMat4<f32> {
        self.projection_matrix
    }

    pub fn get_projection_view_matrix(&self) -> glm::TMat4<f32> {
        self.proj_view_matrix
    }

    pub fn get_frustum(&self) -> &ViewFrustum {
        &self.frustum
    }
}

impl Deref for Camera {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}