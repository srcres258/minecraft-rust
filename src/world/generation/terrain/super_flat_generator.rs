use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};
use crate::world::constants::CHUNK_SIZE;
use crate::world::generation::terrain_generator::TerrainGenerator;

#[derive(Default)]
pub struct SuperFlatGenerator {
}

impl TerrainGenerator for SuperFlatGenerator {
    fn generate_terrain_for(&mut self, chunk: &mut Chunk) {
        for x in 0 .. CHUNK_SIZE as i32 {
            for z in 0 .. CHUNK_SIZE as i32 {
                chunk.set_block(x, 0, z, ChunkBlock::from_block_id(BlockId::Stone));
                chunk.set_block(x, 1, z, ChunkBlock::from_block_id(BlockId::Dirt));
                chunk.set_block(x, 2, z, ChunkBlock::from_block_id(BlockId::Dirt));
                chunk.set_block(x, 3, z, ChunkBlock::from_block_id(BlockId::Dirt));
                chunk.set_block(x, 4, z, ChunkBlock::from_block_id(BlockId::Grass));
            }
        }
    }
    fn minimum_spawn_height(&self) -> i32 {
        1
    }
}