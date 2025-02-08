use crate::camera::Camera;
use crate::renderer::render_master::RenderMaster;
use crate::util::array_2d::Array2D;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::constants::CHUNK_SIZE;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world::World;
use sfml::system::{Vector2i, Vector3i};
use std::sync::{Arc, Mutex};
use crate::util::mem::UWBox;

pub trait IChunk {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock;
    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock);
}

/// @brief A chunk, in other words, a large arrangement of blocks.
pub struct Chunk {
    chunks: Vec<Arc<UWBox<ChunkSection>>>,
    highest_blocks: Array2D<i32, CHUNK_SIZE>,
    location: Vector2i,

    world: Arc<World>,

    is_loaded: bool,
    
    error_section: Option<Arc<UWBox<ChunkSection>>>
}

impl Chunk {
    pub fn new(world: Arc<World>, location: Vector2i) -> Self {
        let mut result = Self {
            chunks: Vec::new(),
            highest_blocks: Array2D::new(),
            location,
            world: Arc::clone(&world),
            is_loaded: false,
            error_section: None
        };
        
        result.highest_blocks.set_all(0);
        result.error_section = Some(ChunkSection::new(
            Vector3i::new(444, 444, 444), world
        ));
        
        result
    }

    pub fn make_mesh(&mut self, camera: &Camera) -> bool {
        for chunk in self.chunks.iter_mut() {
            let mut chunk_obj = chunk.get_mut();
            if !chunk_obj.has_mesh() && camera.frustum().is_box_in_frustum(chunk_obj.aabb()) {
                chunk_obj.make_mesh();
                return true;
            }
        }
        false
    }

    pub fn height_at(&self, x: i32, z: i32) -> i32 {
        if self.out_of_bound(x, 0, z) {
            0
        } else {
            *self.highest_blocks.get(x as usize, z as usize)
        }
    }

    pub fn draw_chunks(&mut self, renderer: &mut RenderMaster, camera: &Camera) {
        for chunk in self.chunks.iter_mut() {
            let mut chunk_obj = chunk.get_mut();
            if chunk_obj.has_mesh() {
                if !chunk_obj.has_buffered() {
                    chunk_obj.buffer_mesh();
                }

                if camera.frustum().is_box_in_frustum(chunk_obj.aabb()) {
                    renderer.draw_chunk(&chunk_obj);
                }
            }
        }
    }

    pub fn has_loaded(&self) -> bool {
        self.is_loaded
    }
    pub fn load(&mut self, generator: &mut dyn TerrainGenerator) {
        if self.has_loaded() {
            return;
        }
        
        generator.generate_terrain_for(self);
        self.is_loaded = true;
    }

    pub fn section(&self, index: i32) -> Arc<UWBox<ChunkSection>> {
        if index >= self.chunks.len() as i32 || index < 0 {
            Arc::clone(self.error_section.as_ref().unwrap())
        } else {
            Arc::clone(&self.chunks[index as usize])
        }
    }

    pub fn location(&self) -> Vector2i {
        self.location
    }

    pub fn delete_meshes(&mut self) {
        self.chunks.iter_mut().for_each(|chunk| chunk.get_mut().delete_meshes());
    }

    fn add_section(&mut self) {
        let y = self.chunks.len() as i32;
        self.chunks.push(ChunkSection::new(
            Vector3i::new(self.location.x, y, self.location.y),
            Arc::clone(&self.world)
        ));
    }
    fn add_section_block_target(&mut self, block_y: i32) {
        let index = block_y / CHUNK_SIZE as i32;
        self.add_section_index_target(index);
    }
    fn add_section_index_target(&mut self, index: i32) {
        while (self.chunks.len() as i32) < index + 1 {
            self.add_section();
        }
    }

    fn out_of_bound(&self, x: i32, y: i32, z: i32) -> bool {
        if x >= CHUNK_SIZE as i32 {
            return true;
        }
        if z >= CHUNK_SIZE as i32 {
            return true;
        }
        
        if x < 0 {
            return true;
        }
        if y < 0 {
            return true;
        }
        if z < 0 {
            return true;
        }
        
        if y >= (self.chunks.len() * CHUNK_SIZE) as i32 {
            return true;
        }
        
        false
    }
}

impl IChunk for Chunk {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        if self.out_of_bound(x, y, z) {
            return ChunkBlock::from_block_id(BlockId::Air);
        }
        
        let by = y % CHUNK_SIZE as i32;
        
        self.chunks[y as usize / CHUNK_SIZE].get().block(x, by, z)
    }

    fn set_block(&mut self, x: i32, mut y: i32, z: i32, block: ChunkBlock) {
        self.add_section_block_target(y);
        if self.out_of_bound(x, y, z) {
            return;
        }

        let by = y % CHUNK_SIZE as i32;
        self.chunks[y as usize / CHUNK_SIZE].get_mut().set_block(x, by, z, block);
        
        if y == *self.highest_blocks.get(x as usize, z as usize) {
            let mut high_block = self.block(x, y, z);
            y -= 1;
            while !high_block.data().is_opaque {
                high_block = self.block(x, y, z);
                y -= 1;
            }
        } else if y > *self.highest_blocks.get(x as usize, z as usize) {
            *self.highest_blocks.get_mut(x as usize, z as usize) = y;
        }
        
        // The rest of code in C++ is commented, hence ignore its implementation.
    }
}