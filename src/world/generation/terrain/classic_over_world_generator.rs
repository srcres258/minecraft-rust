use crate::world::chunk::chunk::Chunk;
use crate::world::generation::terrain_generator::TerrainGenerator;

/// @brief Generates chunks based on perlin noise and recognizable MC parameters.
pub struct ClassicOverWorldGenerator {
    //todo
}

impl ClassicOverWorldGenerator {
    pub fn new() -> Self {
        //todo
    }
}

impl TerrainGenerator for ClassicOverWorldGenerator {
    fn generate_terrain_for(&mut self, chunk: &Chunk) {
        todo!()
    }

    fn minimum_spawn_height(&self) -> i32 {
        todo!()
    }
}