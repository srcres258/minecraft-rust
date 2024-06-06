use crate::world::chunk::chunk::Chunk;

pub trait TerrainGenerator {
    fn generate_terrain_for(&mut self, chunk: &mut Chunk);
    fn get_minimum_spawn_height(&self) -> i32;
}