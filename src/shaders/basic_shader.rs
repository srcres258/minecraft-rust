use std::ffi::CString;
use delegate::delegate;
use gl::types::GLuint;
use nalgebra_glm::Mat4;
use crate::shaders::shader;
use crate::shaders::shader::{Shader, ShaderImpl};

pub trait BasicShader : Shader {
    fn load_projection_view_matrix(&self, pv_matrix: Mat4);
    fn load_model_matrix(&self, matrix: Mat4);
}

pub struct BasicShaderImpl {
    pub base: ShaderImpl,

    pub location_projection_view_matrix: GLuint,
    pub location_model_matrix: GLuint
}

impl BasicShaderImpl {
    pub fn new() -> Self {
        Self::new_ex_1("Basic", "Basic")
    }

    pub fn new_ex_1(vertex_file: &str, fragment_file: &str) -> Self {
        let mut result = Self {
            base: ShaderImpl::new(vertex_file, fragment_file).unwrap(),
            location_projection_view_matrix: GLuint::default(),
            location_model_matrix: GLuint::default()
        };
        result.get_uniforms();
        result
    }

    fn get_uniforms(&mut self) {
        self.use_program();
        unsafe {
            // TODO: test if the program will crash after removing the `c_string` variable as a relay.
            let c_string = CString::new("projViewMatrix").unwrap();
            self.location_projection_view_matrix = gl::GetUniformLocation(self.base.id, c_string.as_ptr()) as _;
            let c_string = CString::new("modelMatrix").unwrap();
            self.location_model_matrix = gl::GetUniformLocation(self.base.id, c_string.as_ptr()) as _;
        }
    }
}

impl Shader for BasicShaderImpl {
    delegate! {
        to self.base {
            fn use_program(&self);
        }
    }
}

impl BasicShader for BasicShaderImpl {
    fn load_projection_view_matrix(&self, pv_matrix: Mat4) {
        shader::load_matrix4(self.location_projection_view_matrix, pv_matrix);
    }
    fn load_model_matrix(&self, matrix: Mat4) {
        shader::load_matrix4(self.location_model_matrix, matrix);
    }
}