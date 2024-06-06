use crate::camera::Camera;
use crate::gl::gl_functions;
use crate::renderer::render_info::RenderInfo;
use crate::shaders::chunk_shader::ChunkShader;
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
        self.chunks.push(*mesh.get_model().get_render_info());
    }

    pub fn render(&mut self, camera: &Camera) {
        if self.chunks.is_empty() {
            return;
        }

        unsafe {
            gl::Disable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);
        }

        self.shader.base.base.use_program();
        BlockDatabase::get().texture_atlas.bind_texture();

        self.shader.base.load_projection_view_matrix(&camera.get_projection_view_matrix());

        for mesh in self.chunks.iter() {
            gl_functions::bind_vao(mesh.vao);
            gl_functions::draw_elements(mesh.indices_count as _);
        }

        self.chunks.clear();
    }
}