use gl::types::GLuint;

#[derive(Copy, Clone)]
pub struct RenderInfo {
    pub vao: GLuint,
    pub indices_count: GLuint
}

impl RenderInfo {
    pub fn reset(&mut self) {
        self.vao = 0;
        self.indices_count = 0;
    }
}

impl Default for RenderInfo {
    fn default() -> Self {
        Self {
            vao: 0,
            indices_count: 0
        }
    }
}