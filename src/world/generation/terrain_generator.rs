use crate::world::chunk::chunk::Chunk;

pub trait TerrainGenerator {
    fn generate_terrain_for(&mut self, chunk: &Chunk);
    fn minimum_spawn_height(&self) -> i32;
}