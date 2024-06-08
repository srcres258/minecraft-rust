use crate::maths::noise_generator::NoiseParameters;
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::biome::biome::{Biome, BiomeBase};
use crate::world::generation::structure::tree_generator;

pub struct GrasslandBiome {
    biome: BiomeBase
}

impl GrasslandBiome {
    pub fn new(seed: i32) -> Self {
        Self {
            biome: BiomeBase::new(Self::get_noise_parameters(), 1000, 20, seed)
        }
    }

    fn get_noise_parameters() -> NoiseParameters {
        let mut height_params = NoiseParameters::default();
        height_params.octaves = 9;
        height_params.amplitude = 85;
        height_params.smoothness = 235;
        height_params.height_offset = -20;
        height_params.roughness = 0.51;

        height_params
    }
}

impl Biome for GrasslandBiome {
    fn get_plant(&self, rand: &Random) -> ChunkBlock {
        if rand.int_in_range(0..=10) > 6 {
            ChunkBlock::new_with_block_id(BlockId::Rose)
        } else {
            ChunkBlock::new_with_block_id(BlockId::TallGrass)
        }
    }

    fn get_top_block(&self, _: &Random) -> ChunkBlock {
        ChunkBlock::new_with_block_id(BlockId::Grass)
    }

    fn get_under_water_block(&self, rand: &Random) -> ChunkBlock {
        if rand.int_in_range(0..=10) > 6 {
            ChunkBlock::new_with_block_id(BlockId::Dirt)
        } else {
            ChunkBlock::new_with_block_id(BlockId::Sand)
        }
    }

    fn get_beach_block(&self, rand: &Random) -> ChunkBlock {
        if rand.int_in_range(0..=10) > 2 {
            ChunkBlock::new_with_block_id(BlockId::Grass)
        } else {
            ChunkBlock::new_with_block_id(BlockId::Dirt)
        }
    }

    fn make_tree(&self, rand: &Random, chunk: &mut Chunk, x: i32, y: i32, z: i32) {
        tree_generator::make_oak_tree(chunk, rand, x, y, z);
    }

    fn get_height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32 {
        self.biome.height_generator.get_height(x, z, chunk_x, chunk_z) as _
    }

    fn get_tree_frequency(&self) -> i32 {
        self.biome.tree_freq
    }

    fn get_plant_frequency(&self) -> i32 {
        self.biome.plant_freq
    }
}