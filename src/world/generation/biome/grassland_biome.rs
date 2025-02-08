use delegate::delegate;
use crate::maths::noise_generator::NoiseParameters;
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::biome::biome::{Biome, BiomeImpl};
use crate::world::generation::structures::tree_generator::make_oak_tree;

pub struct GrasslandBiome {
    base: BiomeImpl
}

impl GrasslandBiome {
    pub fn new(seed: i32) -> Self {
        Self {
            base: BiomeImpl::new(Self::noise_parameters(), 1000, 20, seed)
        }
    }

    fn noise_parameters() -> NoiseParameters {
        let mut result = NoiseParameters::default();
        result.octaves = 9;
        result.amplitude = 85;
        result.smoothness = 235;
        result.height_offset = -20;
        result.roughness = 0.51;
        result
    }
}

impl Biome for GrasslandBiome {
    fn beach_block(&self, rand: &mut Random) -> ChunkBlock {
        if rand.i32_in_range(0, 10) > 2 {
            ChunkBlock::from_block_id(BlockId::Grass)
        } else {
            ChunkBlock::from_block_id(BlockId::Dirt)
        }
    }
    fn plant(&self, rand: &mut Random) -> ChunkBlock {
        if rand.i32_in_range(0, 10) > 6 {
            ChunkBlock::from_block_id(BlockId::Rose)
        } else {
            ChunkBlock::from_block_id(BlockId::TallGrass)
        }
    }
    fn top_block(&self, rand: &mut Random) -> ChunkBlock {
        ChunkBlock::from_block_id(BlockId::Grass)
    }
    fn under_water_block(&self, rand: &mut Random) -> ChunkBlock {
        if rand.i32_in_range(0, 10) > 8 {
            ChunkBlock::from_block_id(BlockId::Dirt)
        } else {
            ChunkBlock::from_block_id(BlockId::Sand)
        }
    }
    fn make_tree(&self, rand: &mut Random, chunk: &mut Chunk, x: i32, y: i32, z: i32) {
        make_oak_tree(chunk, rand, x, y, z);
    }

    delegate! {
        to self.base {
            fn height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32;
            fn tree_frequency(&self) -> i32;
            fn plant_frequency(&self) -> i32;
        }
    }
}