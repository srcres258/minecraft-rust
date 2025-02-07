use std::ffi::CString;
use crate::shaders::basic_shader::{BasicShader, BasicShaderImpl};
use crate::shaders::shader::Shader;
use delegate::delegate;
use gl::types::GLuint;
use nalgebra_glm::Mat4;
use crate::shaders::shader;

pub struct WaterShader {
    base: BasicShaderImpl,

    time: GLuint
}

impl WaterShader {
    pub fn new() -> Self {
        let mut result = Self {
            base: BasicShaderImpl::new_ex_1("Water", "Chunk"),

            time: GLuint::default()
        };
        result.get_uniforms();
        result
    }

    pub fn load_time(&self, time: f32) {
        shader::load_float(self.time, time);
    }

    fn get_uniforms(&mut self) {
        unsafe {
            // TODO: test if the program will crash after removing the `c_string` variable as a relay.
            let c_string = CString::new("globalTime").unwrap();
            self.time = gl::GetUniformLocation(self.base.base.id, c_string.as_ptr()) as _;
        }
    }
}

impl Shader for WaterShader {
    delegate! {
        to self.base.base {
            fn use_program(&self);
        }
    }
}

impl BasicShader for WaterShader {
    delegate! {
        to self.base {
            fn load_projection_view_matrix(&self, pv_matrix: Mat4);
            fn load_model_matrix(&self, matrix: Mat4);
        }
    }
}

impl Default for WaterShader {
    fn default() -> Self {
        Self::new()
    }
}