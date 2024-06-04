use gl::types::GLuint;

#[derive(Copy, Clone, Default)]
pub struct RenderInfo {
    vao: GLuint,
    indices_count: GLuint
}

impl RenderInfo {
    pub fn reset(&mut self) {
        self.vao = 0;
        self.indices_count = 0;
    }
}