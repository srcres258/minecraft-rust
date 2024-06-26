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

use gl::types::{GLfloat, GLuint};
use crate::camera::Camera;
use crate::gl::gl_functions;
use crate::model::Model;
use crate::shaders::skybox_shader::SkyboxShader;
use crate::texture::cube_texture::CubeTexture;

/// @brief Renderer that specifically draws the skybox and entities outside player reach.
pub struct SkyboxRenderer {
    sky_cube: Model,
    shader: SkyboxShader,
    cube_texture: CubeTexture
}

impl SkyboxRenderer {
    pub fn render(&self, camera: &Camera) {
        self.shader.base.use_program();
        self.sky_cube.bind_vao();
        self.cube_texture.bind_texture();
        
        self.shader.load_view_matrix(&camera.get_view_matrix());
        self.shader.load_projection_matrix(&camera.get_proj_matrix());
        
        gl_functions::draw_elements(self.sky_cube.get_indices_count());
    }
}

impl Default for SkyboxRenderer {
    fn default() -> Self {
        let mut result = Self {
            sky_cube: Default::default(),
            shader: Default::default(),
            cube_texture: Default::default()
        };

        const SIZE: GLfloat = 500.;
        let vertex_coords = vec![
            // Back
            SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,

            // Front
            -SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,

            // Right
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            SIZE,

            // Left
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            -SIZE,

            // Top
            -SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,

            // Bottom
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
        ];

        let indices: Vec<GLuint> = vec![
            0,  1,  2,  2,  3,  0,

            4,  5,  6,  6,  7,  4,

            8,  9,  10, 10, 11, 8,

            12, 13, 14, 14, 15, 12,

            16, 17, 18, 18, 19, 16,

            20, 21, 22, 22, 23, 20
        ];

        result.sky_cube.gen_vao();
        result.sky_cube.add_vbo(3, &vertex_coords);
        result.sky_cube.add_ebo(&indices);

        result.cube_texture.load_from_files([
            "dm",
            "dm",
            "dt",
            "db",
            "dm",
            "dm",
        ]);

        result
    }
}