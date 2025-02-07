use crate::application::G_TIME_ELAPSED;
use crate::camera::Camera;
use crate::gl::gl_functions::{bind_vao, draw_elements};
use crate::renderer::render_info::RenderInfo;
use crate::shaders::basic_shader::BasicShader;
use crate::shaders::shader::Shader;
use crate::shaders::water_shader::WaterShader;
use crate::world::chunk::chunk_mesh::ChunkMesh;

/// @brief Renderer specifically targeting water and handling shader behaviors.
#[derive(Default)]
pub struct WaterRenderer {
    chunks: Vec<RenderInfo>,

    shader: WaterShader
}

impl WaterRenderer {
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

        self.shader.load_projection_view_matrix(camera.proj_view_matrix());
        self.shader.load_time(G_TIME_ELAPSED);

        for mesh in self.chunks.iter() {
            bind_vao(mesh.vao);
            draw_elements(mesh.indices_count);
        }

        self.chunks.clear();
    }
}