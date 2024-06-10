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

use crate::application;
use crate::camera::Camera;
use crate::gl::gl_functions;
use crate::renderer::render_info::RenderInfo;
use crate::shaders::water_shader::WaterShader;
use crate::world::chunk::chunk_mesh::ChunkMesh;

/// @brief Renderer specifically targeting water and handling shader behaviors.
#[derive(Default)]
pub struct WaterRenderer {
    chunks: Vec<RenderInfo>,

    shader: WaterShader
}

impl WaterRenderer {
    pub fn add(&mut self, mesh: &ChunkMesh) {
        self.chunks.push(*mesh.get_model().get_render_info());
    }

    pub fn render(&mut self, camera: &Camera) {
        if self.chunks.is_empty() {
            return;
        }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::Disable(gl::CULL_FACE);
        }
        self.shader.base.base.use_program();

        self.shader.base.load_projection_view_matrix(&camera.get_projection_view_matrix());
        unsafe {
            self.shader.load_time(application::TIME_ELAPSED);
        }

        for mesh in self.chunks.iter() {
            gl_functions::bind_vao(mesh.vao);
            gl_functions::draw_elements(mesh.indices_count as _);
        }

        self.chunks.clear();
    }
}