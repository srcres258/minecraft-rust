use gl::types::{GLfloat, GLuint};
use sfml::system::Vector3i;
use crate::mesh::Mesh;
use crate::model::Model;
use crate::world::world_constants::CHUNK_SIZE;

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

impl ChunkMesh {
    pub fn add_face(
        &mut self,
        block_face: [GLfloat; 12],
        texture_coords: [GLfloat; 8],
        chunk_position: &Vector3i,
        block_position: &Vector3i,
        cardinal_light: GLfloat
    ) {
        self.faces += 1;
        let vertices = &mut self.mesh.vertex_positions;
        let tex_coords = &mut self.mesh.texture_coords;
        let indices = &mut self.mesh.indices;

        texture_coords.iter().for_each(|it| {
            tex_coords.push(*it);
        });

        // Vertex: The current vertex in the "blockFace" vector, 4 vertex in total
        // hence "< 4" Index: X, Y, Z
        let mut index = 0;
        for _ in 0..4 {
            vertices.push(block_face[index] + (chunk_position.x * CHUNK_SIZE as i32
                + block_position.x) as f32);
            index += 1;
            vertices.push(block_face[index] + (chunk_position.y * CHUNK_SIZE as i32
                + block_position.y) as f32);
            index += 1;
            vertices.push(block_face[index] + (chunk_position.z * CHUNK_SIZE as i32
                + block_position.z) as f32);
            index += 1;
            self.light.push(cardinal_light);
        }

        indices.push(self.index_index);
        indices.push(self.index_index + 1);
        indices.push(self.index_index + 2);
        indices.push(self.index_index + 2);
        indices.push(self.index_index + 3);
        indices.push(self.index_index);
        self.index_index += 4;
    }

    pub fn buffer_mesh(&mut self) {
        self.model.add_data(&self.mesh);
        self.model.add_vbo(1, &self.light);

        self.mesh.vertex_positions.clear();
        self.mesh.texture_coords.clear();
        self.mesh.indices.clear();
        self.light.clear();

        self.mesh.vertex_positions.shrink_to_fit();
        self.mesh.texture_coords.shrink_to_fit();
        self.mesh.indices.shrink_to_fit();
        self.light.shrink_to_fit();

        self.index_index = 0;
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn delete_data(&mut self) {
        self.model.delete_data();
    }
}

impl ChunkMeshCollection {
    pub fn new(
        solid_mesh: ChunkMesh,
        water_mesh: ChunkMesh,
        flora_mesh: ChunkMesh
    ) -> Self {
        Self { solid_mesh, water_mesh, flora_mesh }
    }
}