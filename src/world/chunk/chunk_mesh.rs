use gl::types::{GLfloat, GLuint};
use sfml::system::Vector3i;
use crate::mesh::Mesh;
use crate::model::Model;
use crate::world::constants::CHUNK_SIZE;

#[derive(Default)]
pub struct ChunkMesh {
    pub faces: i32,

    mesh: Mesh,
    model: Model,
    light: Vec<GLfloat>,
    index: GLuint
}

impl ChunkMesh {
    pub fn add_face(
        &mut self,
        block_face: [GLfloat; 12],
        texture_coords: [GLfloat; 8],
        chunk_position: Vector3i,
        block_position: Vector3i,
        cardinal_light: GLfloat
    ) {
        self.faces += 1;
        let vertices = &mut self.mesh.vertex_positions;
        let tex_coords = &mut self.mesh.texture_coords;
        let indices = &mut self.mesh.indices;

        for coord in texture_coords.iter() {
            tex_coords.push(*coord);
        }

        // Vertex: The current vertex in the "blockFace" vector, 4 vertex in total
        // hence "< 4" Index: X, Y, Z
        let mut index = 0;
        for i in 0 .. 4 {
            vertices.push(block_face[index] +
                chunk_position.x as GLfloat * CHUNK_SIZE as GLfloat +
                block_position.x as GLfloat);
            index += 1;
            vertices.push(block_face[index] +
                chunk_position.y as GLfloat * CHUNK_SIZE as GLfloat +
                block_position.y as GLfloat);
            index += 1;
            vertices.push(block_face[index] +
                chunk_position.z as GLfloat * CHUNK_SIZE as GLfloat +
                block_position.z as GLfloat);
            index += 1;
            self.light.push(cardinal_light);
        }

        indices.push(self.index);
        indices.push(self.index + 1);
        indices.push(self.index + 2);
        indices.push(self.index + 2);
        indices.push(self.index + 3);
        indices.push(self.index);
        self.index += 4;
    }

    pub fn buffer_mesh(&mut self) {
        self.model.add_data(&self.mesh);
        self.model.add_vbo(1, &self.light);

        self.mesh.vertex_positions.clear();
        self.mesh.texture_coords.clear();
        self.mesh.indices.clear();
        self.light.clear();
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn delete_data(&mut self) {
        self.model.delete_data();
    }
}

#[derive(Default)]
pub struct ChunkMeshCollection {
    pub solid_mesh: ChunkMesh,
    pub water_mesh: ChunkMesh,
    pub flora_mesh: ChunkMesh
}