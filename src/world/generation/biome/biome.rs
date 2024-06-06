use crate::maths::noise_generator::{NoiseGenerator, NoiseParameters};
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;

pub trait Biome {
    fn get_plant(&self, rand: &Random) -> ChunkBlock;
    fn get_top_block(&self, rand: &Random) -> ChunkBlock;
    fn get_under_water_block(&self, rand: &Random) -> ChunkBlock;
    fn get_beach_block(&self, _rand: &Random) -> ChunkBlock {
        ChunkBlock::new_with_block_id(BlockId::Sand)
    }
    fn make_tree(&self, rand: &Random, chunk: &mut Chunk, x: i32, y: i32, z: i32);

    fn get_height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32;
    fn get_tree_frequency(&self) -> i32;
    fn get_plant_frequency(&self) -> i32;
}

pub struct BiomeBase {
    pub height_generator: NoiseGenerator,
    pub tree_freq: i32,
    pub plant_freq: i32
}

impl BiomeBase {
    pub fn new(
        parameters: NoiseParameters,
        tree_freq: i32,
        plant_freq: i32,
        seed: i32
    ) -> Self {
        let mut result = Self {
            height_generator: NoiseGenerator::new(seed),
            tree_freq,
            plant_freq
        };
        result.height_generator.set_parameters(parameters);
        result
    }
}