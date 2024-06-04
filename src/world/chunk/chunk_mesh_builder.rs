use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
use crate::world::chunk::chunk_section::ChunkSection;

pub struct ChunkMeshBuilder<'a> {
    p_chunk: &'a ChunkSection,
    p_meshes: &'a ChunkMeshCollection
}

impl<'a> ChunkMeshBuilder<'a> {
    //todo
}