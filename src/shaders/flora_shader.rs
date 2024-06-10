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

use std::ffi::CString;
use gl::types::GLint;
use crate::shaders::basic_shader::BasicShader;
use crate::shaders::shader::{Shader, ShaderBase};

pub struct FloraShader {
    pub base: BasicShader,

    time: GLint
}

impl FloraShader {
    pub fn load_time(&self, time: f32) {
        ShaderBase::load_float(self.time, time);
    }
}

impl Default for FloraShader {
    fn default() -> Self {
        let mut result = Self {
            base: BasicShader::new("Flora", "Chunk"),
            time: 0
        };
        result.get_uniforms();
        result
    }
}

impl Shader for FloraShader {
    fn get_uniforms(&mut self) {
        self.base.get_uniforms();
        unsafe {
            let c_string = CString::new("globalTime").unwrap();
            self.time = gl::GetUniformLocation(self.base.base.id, c_string.as_ptr());
        }
    }
}