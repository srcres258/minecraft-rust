use anyhow::Result;
use gl::types::GLuint;
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use crate::shaders::shader_loader::load_shaders;

pub trait Shader {
    fn use_program(&self);

    // `getUniforms` is defined as protected in C++,
    // hence we don't need to declare it here in Rust trait.
}

pub struct ShaderImpl {
    pub id: GLuint
}

impl ShaderImpl {
    pub fn new(vertex_file: &str, fragment_file: &str) -> Result<Self> {
        let result = Self {
            id: load_shaders(vertex_file, fragment_file)?
        };
        result.use_program();
        Ok(result)
    }
}

impl Shader for ShaderImpl {
    fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderImpl {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub fn load_int(location: GLuint, value: i32) {
    unsafe {
        gl::Uniform1i(location as _, value);
    }
}
pub fn load_float(location: GLuint, value: f32) {
    unsafe {
        gl::Uniform1f(location as _, value);
    }
}

pub fn load_vector2(location: GLuint, vect: Vec2) {
    unsafe {
        gl::Uniform2f(location as _, vect.x, vect.y);
    }
}
pub fn load_vector3(location: GLuint, vect: Vec3) {
    unsafe {
        gl::Uniform3f(location as _, vect.x, vect.y, vect.z);
    }
}
pub fn load_vector4(location: GLuint, vect: Vec4) {
    unsafe {
        gl::Uniform4f(location as _, vect.x, vect.y, vect.z, vect.w);
    }
}

pub fn load_matrix4(location: GLuint, matrix: Mat4) {
    unsafe {
        gl::UniformMatrix4fv(
            location as _,
            1,
            gl::FALSE,
            matrix.as_ptr()
        );
    }
}