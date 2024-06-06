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