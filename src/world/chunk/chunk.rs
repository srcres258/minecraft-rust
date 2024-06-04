use std::cell::RefCell;
use std::rc::Rc;
use sfml::system::Vector2i;
use crate::util::array2d::Array2D;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::world::World;
use crate::world::world_constants::CHUNK_SIZE;

pub trait IChunk {
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock;
    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock);
}

/// @brief A chunk, in other words, a large arrangement of blocks.
pub struct Chunk {
    chunks: Vec<ChunkSection>,
    highest_blocks: Array2D<i32, CHUNK_SIZE>,
    location: Vector2i,

    p_world: Rc<RefCell<World>>,

    is_loaded: bool
}