use crate::world::block::chunk_block::ChunkBlock;

pub trait IChunk {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock;
    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock);
}