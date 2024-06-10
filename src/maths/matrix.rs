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

use crate::camera::Camera;
use crate::config::Config;
use crate::entity::Entity;

pub fn make_model_matrix(entity: &Entity) -> glm::TMat4<f32> {
    let mut matrix: glm::TMat4<f32> = Default::default();

    matrix = glm::rotate(
        &matrix, entity.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0)
    );
    matrix = glm::rotate(
        &matrix, entity.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0)
    );
    matrix = glm::rotate(
        &matrix, entity.rotation.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0)
    );

    matrix = glm::translate(&matrix, &entity.position);

    matrix
}

pub fn make_view_matrix(camera: &Camera) -> glm::TMat4<f32> {
    let mut matrix = glm::diagonal4x4(
        &glm::vec4(1.0f32, 1.0, 1.0, 1.0)
    );

    matrix = glm::rotate(
        &matrix, camera.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0)
    );
    matrix = glm::rotate(
        &matrix, camera.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0)
    );
    matrix = glm::rotate(
        &matrix, camera.rotation.z.to_radians(), &glm::vec3(1.0, 0.0, 1.0)
    );

    matrix = glm::translate(&matrix, &(-camera.position));

    matrix
}

pub fn make_projection_matrix(config: &Config) -> glm::TMat4<f32> {
    let x = config.window_x as f32;
    let y = config.window_y as f32;
    let fov = config.fov as f32;
    
    glm::perspective(fov.to_radians(), x / y, 0.1, 2000.0)
}