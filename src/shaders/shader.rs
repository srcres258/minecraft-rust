extern crate nalgebra_glm as glm;

use gl::types::{GLint, GLuint};
use crate::shaders::shader_loader::load_shaders;

pub trait Shader {
    fn get_uniforms(&mut self);
}

pub struct ShaderBase {
    pub id: GLuint
}

impl ShaderBase {
    pub fn new(vertex_file: &str, fragment_file: &str) -> Self {
        let result = Self {
            id: load_shaders(vertex_file, fragment_file)
        };
        result.use_program();
        result
    }

    pub fn load_int(location: GLint, value: i32) {
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn load_float(location: GLint, value: f32) {
        unsafe {
            gl::Uniform1f(location, value);
        }
    }

    pub fn load_vector_2(location: GLint, vect: &glm::TVec2<f32>) {
        unsafe {
            gl::Uniform2f(location, vect.x, vect.y);
        }
    }

    pub fn load_vector_3(location: GLint, vect: &glm::TVec3<f32>) {
        unsafe {
            gl::Uniform3f(location, vect.x, vect.y, vect.z);
        }
    }

    pub fn load_vector_4(location: GLint, vect: &glm::TVec4<f32>) {
        unsafe {
            gl::Uniform4f(location, vect.x, vect.y, vect.z, vect.w);
        }
    }

    pub fn load_matrix_4(location: GLint, matrix: &glm::TMat4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, &glm::value_ptr(matrix)[0]);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderBase {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}