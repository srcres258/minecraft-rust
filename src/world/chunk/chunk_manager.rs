use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::maths::vector2xz::VectorXZ;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world::World;

pub type ChunkMap = HashMap<VectorXZ, Chunk>;

/// @brief Dynamic chunk manager that affects chunk and block placement.
pub struct ChunkManager {
    chunks: ChunkMap,
    terrain_generator: Box<dyn TerrainGenerator>,

    world: Rc<RefCell<World>>
}