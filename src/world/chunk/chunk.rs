use sfml::system::Vector2i;
use crate::camera::Camera;
use crate::renderer::render_master::RenderMaster;
use crate::util::array_2d::Array2D;
use crate::util::mem::{uw_ref, UWRef};
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::chunk::ichunk::IChunk;
use crate::world::constants::CHUNK_SIZE;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world::World;

/// @brief A chunk, in other words, a large arrangement of blocks.
pub struct Chunk {
    chunks: Vec<ChunkSection>,
    highest_blocks: Array2D<i32, CHUNK_SIZE>,
    location: Vector2i,

    world: UWRef<World>,

    is_loaded: bool
}

impl Chunk {
    pub fn new(world: &World, location: Vector2i) -> Self {
        Self {
            chunks: Vec::new(),
            highest_blocks: Array2D::new(),
            location,
            world: uw_ref(world),
            is_loaded: false
        }
    }

    pub fn make_mesh(&mut self, camera: &Camera) -> bool {
        todo!();
        false
    }

    pub fn height_at(&self, x: i32, z: i32) -> i32 {

    }

    pub fn draw_chunks(&mut self, renderer: &RenderMaster, camera: &Camera) {}

    pub fn has_loaded(&self) -> bool {}
    pub fn load(&mut self, generator: &dyn TerrainGenerator) {}

    pub fn section(&self, index: i32) -> &ChunkSection {}

    pub fn location(&self) -> Vector2i {}

    pub fn delete_meshes(&mut self) {}

    fn add_section(&mut self) {}
    fn add_section_block_target(&mut self, block_y: i32) {}
    fn add_section_index_target(&mut self, index: i32) {}

    fn out_of_bound(&self, x: i32, y: i32, z: i32) -> bool {}
}

impl IChunk for Chunk {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        todo!()
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        self.add_section_block_target(y);
        if self.out_of_bound(x, y, z) {
            return;
        }

        let by = y as usize % CHUNK_SIZE;
        self.chunks[y / CHUNK_SIZE]//todo
    }
}