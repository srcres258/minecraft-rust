use gl::types::{GLfloat, GLuint};

/// @brief Mesh struct used for the purpose of constructing block meshes.
#[derive(Clone, Default)]
pub struct Mesh {
    pub vertex_positions: Vec<GLfloat>,
    pub texture_coords: Vec<GLfloat>,
    pub indices: Vec<GLuint>
}