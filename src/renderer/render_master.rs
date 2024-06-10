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

use sfml::window::Window;
use crate::camera::Camera;
use crate::renderer::chunk_renderer::ChunkRenderer;
use crate::renderer::flora_renderer::FloraRenderer;
use crate::renderer::skybox_renderer::SkyboxRenderer;
use crate::renderer::water_renderer::WaterRenderer;
use crate::world::chunk::chunk_section::ChunkSection;

/// @brief Master rendering class that handles the sum of drawn in-game objects.
#[derive(Default)]
pub struct RenderMaster {
    // Chunks
    chunk_renderer: ChunkRenderer,
    water_renderer: WaterRenderer,
    flora_renderer: FloraRenderer,

    // Detail
    skybox_renderer: SkyboxRenderer,
    
    draw_box: bool
}

impl RenderMaster {
    pub fn draw_chunk(&mut self, chunk: &ChunkSection) {
        let solid_mesh = &chunk.get_meshes().solid_mesh;
        let water_mesh = &chunk.get_meshes().water_mesh;
        let flora_mesh = &chunk.get_meshes().flora_mesh;

        if solid_mesh.faces > 0 {
            self.chunk_renderer.add(solid_mesh);
        }

        if water_mesh.faces > 0 {
            self.water_renderer.add(water_mesh);
        }

        if flora_mesh.faces > 0 {
            self.flora_renderer.add(flora_mesh);
        }
    }

    pub fn draw_sky(&mut self) {
        self.draw_box = true;
    }

    pub fn finish_render(&mut self, window: &mut Window, camera: &Camera) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
        }
        
        self.chunk_renderer.render(camera);
        self.water_renderer.render(camera);
        self.flora_renderer.render(camera);

        if self.draw_box {
            unsafe {
                gl::Disable(gl::CULL_FACE);
            }
            self.skybox_renderer.render(camera);
            self.draw_box = false;
        }

        window.display();
    }
}