use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world_constants::CHUNK_SIZE;

#[derive(Default)]
pub struct SuperFlatGenerator {
}

impl TerrainGenerator for SuperFlatGenerator {
    fn generate_terrain_for(&mut self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.set_block(x as _, 0, z as _, ChunkBlock::new_with_block_id(BlockId::Stone));
                chunk.set_block(x as _, 1, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 2, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 3, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 4, z as _, ChunkBlock::new_with_block_id(BlockId::Grass));
            }
        }
    }

    fn get_minimum_spawn_height(&self) -> i32 {
        1
    }
}