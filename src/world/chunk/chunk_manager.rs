use std::collections::HashMap;
use crate::maths::vector2xz::VectorXZ;
use crate::world::world::World;

pub type ChunkMap = HashMap<VectorXZ, Chunk>;

/// @brief Dynamic chunk manager that affects chunk and block placement.
pub struct ChunkManager<'a> {
    chunks: ChunkMap,
    terrain_generator: Box<TerrainGenerator>,

    world: &'a World
}