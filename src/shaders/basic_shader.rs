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

use std::ffi::CString;
use gl::types::GLint;
use crate::shaders::shader::{Shader, ShaderBase};

pub struct BasicShader {
    pub base: ShaderBase,

    location_projection_view_matrix: GLint,
    location_model_matrix: GLint
}

impl BasicShader {
    pub fn new(vertex_file: &str, fragment_file: &str) -> Self {
        Self {
            base: ShaderBase::new(vertex_file, fragment_file),
            location_projection_view_matrix: 0,
            location_model_matrix: 0
        }
    }

    pub fn load_projection_view_matrix(&self, pv_matrix: &glm::TMat4<f32>) {
        ShaderBase::load_matrix_4(self.location_projection_view_matrix, pv_matrix);
    }

    pub fn load_model_matrix(&self, matrix: &glm::TMat4<f32>) {
        ShaderBase::load_matrix_4(self.location_model_matrix, matrix);
    }
}

impl Default for BasicShader {
    fn default() -> Self {
        Self {
            base: ShaderBase::new("Basic", "Basic"),
            location_projection_view_matrix: 0,
            location_model_matrix: 0
        }
    }
}

impl Shader for BasicShader {
    fn get_uniforms(&mut self) {
        self.base.use_program();
        unsafe {
            let c_string = CString::new("projViewMatrix").unwrap();
            self.location_projection_view_matrix = gl::GetUniformLocation(self.base.id, c_string.as_ptr());
            let c_string = CString::new("modelMatrix").unwrap();
            self.location_model_matrix = gl::GetUniformLocation(self.base.id, c_string.as_ptr());
        }
    }
}