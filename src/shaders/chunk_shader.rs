use delegate::delegate;
use nalgebra_glm::Mat4;
use crate::shaders::basic_shader::{BasicShader, BasicShaderImpl};
use crate::shaders::shader::Shader;

pub struct ChunkShader {
    base: BasicShaderImpl
}

impl ChunkShader {
    pub fn new() -> Self {
        Self {
            base: BasicShaderImpl::new_ex_1("Chunk", "Chunk")
        }
    }
}

impl Shader for ChunkShader {
    delegate! {
        to self.base.base {
            fn use_program(&self);
        }
    }
}

impl BasicShader for ChunkShader {
    delegate! {
        to self.base {
            fn load_projection_view_matrix(&self, pv_matrix: Mat4);
            fn load_model_matrix(&self, matrix: Mat4);
        }
    }
}

impl Default for ChunkShader {
    fn default() -> Self {
        Self::new()
    }
}