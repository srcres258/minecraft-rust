use crate::camera::Camera;
use crate::gl::gl_functions::{bind_vao, draw_elements};
use crate::renderer::render_info::RenderInfo;
use crate::shaders::basic_shader::BasicShader;
use crate::shaders::chunk_shader::ChunkShader;
use crate::shaders::shader::Shader;
use crate::texture::basic_texture::BasicTexture;
use crate::world::block::block_database::BlockDatabase;
use crate::world::chunk::chunk_mesh::ChunkMesh;

/// @brief Block chunk renderer that helps display block data.
#[derive(Default)]
pub struct ChunkRenderer {
    chunks: Vec<RenderInfo>,

    shader: ChunkShader
}

impl ChunkRenderer {
    pub fn add(&mut self, mesh: &ChunkMesh) {
        self.chunks.push(mesh.model().render_info());
    }
    pub fn render(&mut self, camera: &Camera) {
        if self.chunks.is_empty() {
            return;
        }

        unsafe {
            gl::Disable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);
        }

        self.shader.use_program();
        BlockDatabase::get().texture_atlas.bind_texture();

        self.shader.load_projection_view_matrix(camera.proj_view_matrix());

        for mesh in self.chunks.iter() {
            bind_vao(mesh.vao);
            draw_elements(mesh.indices_count as _);
        }

        self.chunks.clear();
    }
}