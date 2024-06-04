use gl::types::{GLfloat, GLuint};

#[derive(Clone, Default)]
pub struct ChunkMesh {
    pub faces: i32,

    mesh: Mesh,
    model: Model,
    light: Vec<GLfloat>,
    index_index: GLuint
}

#[derive(Clone, Default)]
pub struct ChunkMeshCollection {
    pub solid_mesh: ChunkMesh,
    pub water_mesh: ChunkMesh,
    pub flora_mesh: ChunkMesh
}