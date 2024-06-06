use crate::shaders::basic_shader::BasicShader;
use crate::shaders::shader::Shader;

pub struct ChunkShader {
    pub base: BasicShader
}

impl Default for ChunkShader {
    fn default() -> Self {
        let mut result = Self {
            base: BasicShader::new("Chunk", "Chunk")
        };
        result.get_uniforms();
        result
    }
}

impl Shader for ChunkShader {
    fn get_uniforms(&mut self) {
        self.base.get_uniforms();
    }
}