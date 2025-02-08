use delegate::delegate;
use crate::maths::noise_generator::NoiseParameters;
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::biome::biome::{Biome, BiomeImpl};
use crate::world::generation::structures::tree_generator::{make_oak_tree, make_palm_tree};

pub struct OceanBiome {
    base: BiomeImpl
}

impl OceanBiome {
    pub fn new(seed: i32) -> Self {
        Self {
            base: BiomeImpl::new(Self::noise_parameters(), 50, 100, seed)
        }
    }

    fn noise_parameters() -> NoiseParameters {
        let mut result = NoiseParameters::default();
        result.octaves = 7;
        result.amplitude = 43;
        result.smoothness = 55;
        result.height_offset = 0;
        result.roughness = 0.50;
        result
    }
}

impl Biome for OceanBiome {
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
        ChunkBlock::from_block_id(BlockId::Sand)
    }
    fn make_tree(&self, rand: &mut Random, chunk: &mut Chunk, x: i32, y: i32, z: i32) {
        if rand.i32_in_range(0, 5) < 3 {
            make_palm_tree(chunk, rand, x, y, z);
        } else {
            make_oak_tree(chunk, rand, x, y, z);
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