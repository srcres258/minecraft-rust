use delegate::delegate;
use crate::maths::noise_generator::NoiseParameters;
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;
use crate::world::constants::WATER_LEVEL;
use crate::world::generation::biome::biome::{Biome, BiomeImpl};
use crate::world::generation::structures::tree_generator::{make_cactus, make_palm_tree};

pub struct DesertBiome {
    base: BiomeImpl
}

impl DesertBiome {
    pub fn new(seed: i32) -> Self {
        Self {
            base: BiomeImpl::new(Self::noise_parameters(), 1350, 500, seed)
        }
    }

    fn noise_parameters() -> NoiseParameters {
        let mut result = NoiseParameters::default();
        result.octaves = 9;
        result.amplitude = 80;
        result.smoothness = 335;
        result.height_offset = -7;
        result.roughness = 0.56;
        result
    }
}

impl Biome for DesertBiome {
    fn plant(&self, rand: &mut Random) -> ChunkBlock {
        ChunkBlock::from_block_id(BlockId::DeadShrub)
    }
    fn top_block(&self, rand: &mut Random) -> ChunkBlock {
        ChunkBlock::from_block_id(BlockId::Sand)
    }
    fn under_water_block(&self, rand: &mut Random) -> ChunkBlock {
        ChunkBlock::from_block_id(BlockId::Sand)
    }
    fn make_tree(&self, rand: &mut Random, chunk: &mut Chunk, x: i32, y: i32, z: i32) {
        if y < WATER_LEVEL as i32 + 15 {
            if rand.i32_in_range(0, 100) > 75 {
                make_palm_tree(chunk, rand, x, y, z);
            } else {
                make_cactus(chunk, rand, x, y, z);
            }
        } else {
            make_cactus(chunk, rand, x, y, z);
        }
    }

    delegate! {
        to self.base {
            fn beach_block(&self, rand: &mut Random) -> ChunkBlock;

            fn height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32;
            fn tree_frequency(&self) -> i32;
            fn plant_frequency(&self) -> i32;
        }
    }
}