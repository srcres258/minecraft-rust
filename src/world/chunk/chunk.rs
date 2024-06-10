// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use sfml::system::{Vector2i, Vector3i};
use crate::camera::Camera;
use crate::renderer::render_master::RenderMaster;
use crate::util::array2d::Array2D;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world::World;
use crate::world::world_constants::CHUNK_SIZE;

pub trait IChunk {
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock;
    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock);
}

/// @brief A chunk, in other words, a large arrangement of blocks.
pub struct Chunk {
    chunks: Vec<ChunkSection>,
    highest_blocks: Array2D<i32>,
    location: Vector2i,

    p_world: Arc<UnsafeCellWrapper<World>>,

    is_loaded: bool,

    error_section: ChunkSection
}

impl Chunk {
    pub fn new(world: Arc<UnsafeCellWrapper<World>>, location: Vector2i) -> Self {
        let mut result = Self {
            chunks: Vec::new(),
            highest_blocks: Array2D::new(CHUNK_SIZE),
            location,
            p_world: Arc::clone(&world),
            is_loaded: false,
            error_section: ChunkSection::new(Vector3i::new(444, 444, 444), world)
        };
        result.highest_blocks.set_all(0);
        result
    }

    pub fn make_mesh(&mut self, camera: &Camera) -> bool {
        for chunk in self.chunks.iter_mut() {
            if !chunk.has_mesh() && camera.get_frustum().is_box_in_frustum(chunk.aabb) {
                chunk.make_mesh();
                return true;
            }
        }
        false
    }

    pub fn get_height_at(&self, x: i32, z: i32) -> i32 {
        *self.highest_blocks.get(x as _, z as _)
    }

    pub fn draw_chunks(&mut self, renderer: &mut RenderMaster, camera: &Camera) {
        for chunk in self.chunks.iter_mut() {
            if chunk.has_mesh() {
                if !chunk.has_buffered() {
                    chunk.buffer_mesh();
                }

                if camera.get_frustum().is_box_in_frustum(chunk.aabb) {
                    renderer.draw_chunk(chunk);
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

    pub fn get_section(&self, index: i32) -> &ChunkSection {
        if index >= self.chunks.len() as i32 || index < 0 {
            &self.error_section
        } else {
            &self.chunks[index as usize]
        }
    }

    pub fn get_section_mut(&mut self, index: i32) -> &mut ChunkSection {
        if index >= self.chunks.len() as i32 || index < 0 {
            &mut self.error_section
        } else {
            &mut self.chunks[index as usize]
        }
    }

    pub fn get_location(&self) -> Vector2i {
        self.location
    }

    pub fn delete_meshes(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.delete_meshes();
        }
    }

    fn add_section(&mut self) {
        let y = self.chunks.len();
        self.chunks.push(ChunkSection::new(
            Vector3i::new(self.location.x, y as _, self.location.y),
            Arc::clone(&self.p_world)
        ));
    }

    fn add_sections_block_target(&mut self, block_y: i32) {
        let index = block_y / CHUNK_SIZE as i32;
        self.add_sections_index_target(index);
    }

    fn add_sections_index_target(&mut self, index: i32) {
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
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        if self.out_of_bound(x, y, z) {
            return ChunkBlock::new_with_block_id(BlockId::Air);
        }

        let b_y = y % CHUNK_SIZE as i32;

        self.chunks[(y / CHUNK_SIZE as i32) as usize].get_block(x, b_y, z)
    }

    fn set_block(&mut self, x: i32, mut y: i32, z: i32, block: ChunkBlock) {
        self.add_sections_block_target(y);
        if self.out_of_bound(x, y, z) {
            return;
        }

        let b_y = y % CHUNK_SIZE as i32;
        self.chunks[(y / CHUNK_SIZE as i32) as usize].set_block(x, b_y, z, block);

        if y == *self.highest_blocks.get(x as _, z as _) {
            let mut high_block = self.get_block(x, y, z);
            y -= 1;
            while !high_block.get_data().read().unwrap().block_data().is_opaque {
                high_block = self.get_block(x, y, z);
                y -= 1;
            }
        } else if y > *self.highest_blocks.get(x as _, z as _) {
            *self.highest_blocks.get_mut(x as _, z as _) = y;
        }
    }
}