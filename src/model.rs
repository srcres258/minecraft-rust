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

use std::{mem, ptr};
use gl::types::{GLfloat, GLuint};
use crate::mesh::Mesh;
use crate::renderer::render_info::RenderInfo;

/// @brief Models using mesh data to spawn entities for the game world.
#[derive(Clone, Default)]
pub struct Model {
    render_info: RenderInfo,

    vbo_count: i32,
    buffers: Vec<GLuint>
}

impl Model {
    /// @brief Default constructor.
    /// @param mesh
    pub fn new(mesh: &Mesh) -> Self {
        let mut result = Self::default();
        result.add_data(mesh);
        result
    }

    pub fn add_data(&mut self, mesh: &Mesh) {
        self.gen_vao();

        self.add_vbo(3, &mesh.vertex_positions);
        self.add_vbo(2, &mesh.texture_coords);
        self.add_ebo(&mesh.indices);
    }

    /// @brief Deletes model data, used to free models from memory.
    pub fn delete_data(&mut self) {
        unsafe {
            if self.render_info.vao != 0 {
                gl::DeleteVertexArrays(1, &self.render_info.vao);
            }
            if self.buffers.len() > 0 {
                gl::DeleteBuffers(self.buffers.len() as _, self.buffers.as_ptr());
            }
        }

        self.buffers.clear();

        self.vbo_count = 0;
        self.render_info.reset();
    }

    pub fn gen_vao(&mut self) {
        if self.render_info.vao != 0 {
            self.delete_data();
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.render_info.vao);
            gl::BindVertexArray(self.render_info.vao);
        }
    }

    pub fn add_ebo(&mut self, indices: &Vec<GLuint>) {
        self.render_info.indices_count = indices.len() as _;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as _,
                indices.as_ptr() as _,
                gl::STATIC_DRAW
            );
        }
    }

    pub fn add_vbo(&mut self, dimensions: i32, data: &Vec<GLfloat>) {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as _,
                data.as_ptr() as _,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(
                self.vbo_count as _,
                dimensions,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null()
            );

            gl::EnableVertexAttribArray(self.vbo_count as _);
            self.vbo_count += 1;
        }

        self.buffers.push(vbo);
    }

    pub fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.render_info.vao);
        }
    }

    pub fn get_indices_count(&self) -> i32 {
        self.render_info.indices_count as _
    }

    pub fn get_render_info(&self) -> &RenderInfo {
        &self.render_info
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self.delete_data();
    }
}