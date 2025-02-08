use crate::maths::noise_generator::{NoiseGenerator, NoiseParameters};
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::Chunk;

pub trait Biome {
    fn plant(&self, rand: &mut Random) -> ChunkBlock;
    fn top_block(&self, rand: &mut Random) -> ChunkBlock;
    fn under_water_block(&self, rand: &mut Random) -> ChunkBlock;
    fn beach_block(&self, rand: &mut Random) -> ChunkBlock;
    fn make_tree(&self, rand: &mut Random, chunk: &mut Chunk, x: i32, y: i32, z: i32);

    fn height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32;
    fn tree_frequency(&self) -> i32;
    fn plant_frequency(&self) -> i32;
}

pub struct BiomeImpl {
    pub height_generator: NoiseGenerator,
    pub tree_freq: i32,
    pub plant_freq: i32
}

impl BiomeImpl {
    pub fn new(parameters: NoiseParameters, tree_freq: i32, plant_freq: i32, seed: i32) -> Self {
        let mut result = Self {
            height_generator: NoiseGenerator::new(seed),
            tree_freq,
            plant_freq
        };
        result.height_generator.set_parameters(parameters);
        result
    }

    pub fn beach_block(&self, rand: &mut Random) -> ChunkBlock {
        ChunkBlock::from_block_id(BlockId::Sand)
    }

    pub fn height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> i32 {
        self.height_generator.height(x, z, chunk_x, chunk_z) as _
    }
    pub fn tree_frequency(&self) -> i32 {
        self.tree_freq
    }
    pub fn plant_frequency(&self) -> i32 {
        self.plant_freq
    }
}