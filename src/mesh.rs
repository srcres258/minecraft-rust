use gl::types::{GLfloat, GLuint};

/// @brief Mesh struct used for the purpose of constructing block meshes.
#[derive(Clone)]
pub struct Mesh {
    pub vertex_positions: Vec<GLfloat>,
    pub texture_coords: Vec<GLfloat>,
    pub indices: Vec<GLuint>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertex_positions: Vec::new(),
            texture_coords: Vec::new(),
            indices: Vec::new()
        }
    }
}