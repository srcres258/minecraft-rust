use std::ptr;
use gl::types::{GLsizei, GLuint};

pub fn draw_elements(indices_count: GLsizei) {
    unsafe {
        gl::DrawElements(gl::TRIANGLES, indices_count, gl::UNSIGNED_INT, ptr::null());
    }
}

pub fn bind_vao(vao: GLuint) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}