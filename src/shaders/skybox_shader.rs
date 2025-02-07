use crate::shaders::shader;
use crate::shaders::shader::{Shader, ShaderImpl};
use delegate::delegate;
use gl::types::GLuint;
use nalgebra_glm::Mat4;
use std::ffi::CString;

pub struct SkyboxShader {
    base: ShaderImpl,

    location_projection: GLuint,
    location_view: GLuint
}

impl SkyboxShader {
    pub fn new() -> Self {
        let mut result = Self {
            base: ShaderImpl::new("Skybox", "Skybox").unwrap(),
            location_projection: GLuint::default(),
            location_view: GLuint::default()
        };
        result.get_uniforms();
        result
    }

    pub fn load_view_matrix(&self, mut view_matrix: Mat4) {
        // Note that the index order in nalgebra_glm lib in Rust is **reversed**
        // compared to glm lib in C++.
        //
        // e.g.
        // In C++: viewMatrix[3][0] = 0;
        // In Rust: view_matrix[(0, 3)] = 0.;

        view_matrix[(0, 3)] = 0.;
        view_matrix[(1, 3)] = 0.;
        view_matrix[(2, 3)] = 0.;
        shader::load_matrix4(self.location_view, view_matrix);
    }

    pub fn load_projection_matrix(&self, proj: Mat4) {
        shader::load_matrix4(self.location_projection, proj);
    }

    fn get_uniforms(&mut self) {
        unsafe {
            // TODO: test if the program will crash after removing the `c_string` variable as a relay.
            let c_string = CString::new("projectionMatrix").unwrap();
            self.location_projection = gl::GetUniformLocation(self.base.id, c_string.as_ptr()) as _;
            let c_string = CString::new("viewMatrix").unwrap();
            self.location_view = gl::GetUniformLocation(self.base.id, c_string.as_ptr()) as _;
        }
    }
}

impl Shader for SkyboxShader {
    delegate! {
        to self.base {
            fn use_program(&self);
        }
    }
}

impl Default for SkyboxShader {
    fn default() -> Self {
        Self::new()
    }
}