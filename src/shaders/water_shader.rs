use std::ffi::CString;
use gl::types::GLint;
use crate::shaders::basic_shader::BasicShader;
use crate::shaders::shader::{Shader, ShaderBase};

pub struct WaterShader {
    pub base: BasicShader,

    time: GLint
}

impl WaterShader {
    pub fn load_time(&self, time: f32) {
        ShaderBase::load_float(self.time, time);
    }
}

impl Default for WaterShader {
    fn default() -> Self {
        let mut result = Self {
            base: BasicShader::new("Water", "Chunk"),
            time: 0
        };
        result.get_uniforms();
        result
    }
}

impl Shader for WaterShader {
    fn get_uniforms(&mut self) {
        self.base.get_uniforms();
        unsafe {
            let c_string = CString::new("globalTime").unwrap();
            self.time = gl::GetUniformLocation(self.base.base.id, c_string.as_ptr());
        }
    }
}