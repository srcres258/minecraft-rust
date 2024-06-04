pub trait IChunk {
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock;
    fn set_block(&self, x: i32, y: i32, z: i32, block: ChunkBlock);
}