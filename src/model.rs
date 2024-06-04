use gl::types::{GLfloat, GLuint};
use crate::mesh::Mesh;
use crate::renderer::render_info::RenderInfo;

/// @brief Models using mesh data to spawn entities for the game world.
#[derive(Clone, Default)]
pub struct Model {
    render_info: RenderInfo,

    vbo_count: i32,
    buffers: Vec<GLuint>
}

impl Model {
    pub fn new(mesh: &Mesh) -> Self {
        let mut result = Self::default();
        result.add_data(mesh);
        result
    }

    pub fn add_data(&mut self, mesh: &Mesh) {
        self.gen_vao();

        self.add_vbo(3, &mesh.vertex_positions);
        self.add_vbo(2, &mesh.texture_coords);
        self.add_ebo(&mesh.indices);
    }

    pub fn delete_data(&mut self) {
        //todo
    }

    pub fn gen_vao(&mut self) {
        //todo
    }

    pub fn add_ebo(&mut self, indices: &Vec<GLuint>) {
        //todo
    }

    pub fn add_vbo(&mut self, dimensions: i32, data: &Vec<GLfloat>) {
        //todo
    }

    pub fn bind_vao(&self) {
        //todo
    }

    pub fn get_indices_count(&self) -> i32 {
        //todo
    }

    pub fn get_render_info(&self) -> &RenderInfo {
        //todo
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self.delete_data();
    }
}