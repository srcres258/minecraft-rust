use crate::application;
use crate::camera::Camera;
use crate::gl::gl_functions;
use crate::renderer::render_info::RenderInfo;
use crate::shaders::flora_shader::FloraShader;
use crate::world::chunk::chunk_mesh::ChunkMesh;

/// @brief Renderer handling 'flora' based entities that are not true blocks.
#[derive(Default)]
pub struct FloraRenderer {
    chunks: Vec<RenderInfo>,

    shader: FloraShader
}

impl FloraRenderer {
    pub fn add(&mut self, mesh: &ChunkMesh) {
        self.chunks.push(*mesh.get_model().get_render_info());
    }

    pub fn render(&mut self, camera: &Camera) {
        if self.chunks.is_empty() {
            return;
        }

        unsafe {
            gl::Disable(gl::BLEND);
            gl::Disable(gl::CULL_FACE);
        }
        self.shader.base.base.use_program();

        self.shader.base.load_projection_view_matrix(&camera.get_projection_view_matrix());
        unsafe {
            self.shader.load_time(application::TIME_ELAPSED);
        }

        for mesh in self.chunks.iter() {
            gl_functions::bind_vao(mesh.vao);
            gl_functions::draw_elements(mesh.indices_count as _);
        }

        self.chunks.clear();
    }
}