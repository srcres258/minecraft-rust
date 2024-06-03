use crate::world::world::World;

/// @brief Dynamic chunk manager that affects chunk and block placement.
pub struct ChunkManager<'a> {
    chunks: ChunkMap,
    terrain_generator: Box<TerrainGenerator>,

    world: &'a World
}