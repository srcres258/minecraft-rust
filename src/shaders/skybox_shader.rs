extern crate nalgebra_glm as glm;

use std::ffi::CString;
use gl::types::GLint;
use crate::shaders::shader::{Shader, ShaderBase};

pub struct SkyboxShader {
    pub base: ShaderBase,

    location_projection: GLint,
    location_view: GLint
}

impl SkyboxShader {
    pub fn new(vertex_file: &str, fragment_file: &str) -> Self {
        Self {
            base: ShaderBase::new(vertex_file, fragment_file),
            location_projection: 0,
            location_view: 0
        }
    }

    pub fn load_view_matrix(&self, view_matrix: &glm::TMat4<f32>) {
        let mut view_matrix = *view_matrix;
        view_matrix[(3, 0)] = 0.;
        view_matrix[(3, 1)] = 0.;
        view_matrix[(3, 2)] = 0.;
        ShaderBase::load_matrix_4(self.location_view, &view_matrix);
    }

    pub fn load_projection_matrix(&self, proj: &glm::TMat4<f32>) {
        ShaderBase::load_matrix_4(self.location_projection, proj);
    }
}

impl Default for SkyboxShader {
    fn default() -> Self {
        Self {
            base: ShaderBase::new("Skybox", "Skybox"),
            location_projection: 0,
            location_view: 0
        }
    }
}

impl Shader for SkyboxShader {
    fn get_uniforms(&mut self) {
        unsafe {
            let c_string = CString::new("projectionMatrix").unwrap();
            self.location_projection = gl::GetUniformLocation(self.base.id, c_string.as_ptr());
            let c_string = CString::new("viewMatrix").unwrap();
            self.location_view = gl::GetUniformLocation(self.base.id, c_string.as_ptr());
        }
    }
}