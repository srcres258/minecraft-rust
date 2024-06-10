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

extern crate nalgebra_glm as glm;

use std::slice::Iter;
use std::sync::Arc;
use sfml::system::Vector3i;
use crate::physics::aabb::AABB;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
use crate::world::chunk::chunk_mesh_builder::ChunkMeshBuilder;
use crate::world::world::World;
use crate::world::world_constants::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME};

#[derive(Copy, Clone, Default)]
pub struct Layer {
    solid_block_count: i32
}

pub struct ChunkSection {
    pub(crate) blocks: [ChunkBlock; CHUNK_VOLUME],
    layers: [Layer; CHUNK_SIZE],

    pub(crate) meshes: ChunkMeshCollection,
    pub aabb: AABB,
    pub(crate) location: Vector3i,

    p_world: Arc<UnsafeCellWrapper<World>>,

    has_mesh: bool,
    has_buffered_mesh: bool
}

impl Layer {
    pub fn update(&mut self, c: ChunkBlock) {
        if c.get_data().read().unwrap().block_data().is_opaque {
            self.solid_block_count -= 1;
        } else {
            self.solid_block_count += 1;
        }
    }

    pub fn is_all_solid(&self) -> bool {
        self.solid_block_count == CHUNK_AREA as _
    }
}

impl ChunkSection {
    pub fn new(
        location: Vector3i,
        world: Arc<UnsafeCellWrapper<World>>
    ) -> Self {
        let mut result = Self {
            blocks: [ChunkBlock::default(); CHUNK_VOLUME],
            layers: Default::default(),
            meshes: Default::default(),
            aabb: AABB::new(&glm::vec3(CHUNK_SIZE as _, CHUNK_SIZE as _, CHUNK_SIZE as _)),
            location,
            p_world: world,
            has_mesh: false,
            has_buffered_mesh: false
        };
        result.aabb.update(&glm::vec3(
            (location.x * CHUNK_SIZE as i32) as _,
            (location.y * CHUNK_SIZE as i32) as _,
            (location.z * CHUNK_SIZE as i32) as _
        ));
        result
    }
    
    pub fn get_location(&self) -> Vector3i {
        self.location
    }
    
    pub fn has_mesh(&self) -> bool {
        self.has_mesh
    }
    
    pub fn has_buffered(&self) -> bool {
        self.has_buffered_mesh
    }
    
    pub fn make_mesh(&mut self) {
        ChunkMeshBuilder::new(self).build_mesh();
        self.has_mesh = true;
        self.has_buffered_mesh = false;
    }
    
    pub fn buffer_mesh(&mut self) {
        self.meshes.solid_mesh.buffer_mesh();
        self.meshes.water_mesh.buffer_mesh();
        self.meshes.flora_mesh.buffer_mesh();
        self.has_buffered_mesh = true;
    }
    
    pub fn exec_on_layer<R>(&self, y: i32, func: impl FnOnce(&Layer) -> R) -> R {
        let p_world;
        unsafe {
            p_world = &mut *self.p_world.get();
        }
        if y == -1 {
            p_world.get_chunk_manager_mut()
                .get_chunk(self.location.x, self.location.z)
                .get_section(self.location.y - 1)
                .exec_on_layer(CHUNK_SIZE as i32 - 1, func)
        } else if y == CHUNK_SIZE as i32 {
            p_world.get_chunk_manager_mut()
                .get_chunk(self.location.x, self.location.z)
                .get_section(self.location.y + 1)
                .exec_on_layer(0, func)
        } else {
            func(&self.layers[y as usize])
        }
    }
    
    pub fn get_adjacent(&self, dx: i32, dz: i32) -> &ChunkSection {
        let new_x = self.location.x + dx;
        let new_z = self.location.z + dz;

        unsafe {
            (*self.p_world.get()).get_chunk_manager_mut()
                .get_chunk(new_x, new_z)
                .get_section(self.location.y)
        }
    }
    
    pub fn get_meshes(&self) -> &ChunkMeshCollection {
        &self.meshes
    }
    
    pub fn delete_meshes(&mut self) {
        if self.has_mesh {
            self.has_buffered_mesh = false;
            self.has_mesh = false;
            self.meshes.solid_mesh.delete_data();
            self.meshes.water_mesh.delete_data();
            self.meshes.flora_mesh.delete_data();
        }
    }
    
    pub fn iter(&self) -> Iter<'_, ChunkBlock> {
        self.blocks.iter()
    }

    fn to_world_position(&self, x: i32, y: i32, z: i32) -> Vector3i {
        Vector3i::new(
            self.location.x * CHUNK_SIZE as i32 + x,
            self.location.y * CHUNK_SIZE as i32 + y,
            self.location.z * CHUNK_SIZE as i32 + z
        )
    }

    fn out_of_bounds(value: i32) -> bool {
        value >= CHUNK_SIZE as i32 || value < 0
    }

    fn get_index(x: i32, y: i32, z: i32) -> i32 {
        y * CHUNK_AREA as i32 + z * CHUNK_SIZE as i32 + x
    }
}

impl IChunk for ChunkSection {
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            unsafe {
                let p_world = &mut *self.p_world.get();
                return p_world.get_block(location.x, location.y, location.z);
            }
        }

        self.blocks[Self::get_index(x, y, z) as usize]
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            unsafe {
                let p_world = &mut *self.p_world.get();
                p_world.set_block(location.x, location.y, location.z, block);
            }
            return;
        }

        self.layers[y as usize].update(block);

        self.blocks[Self::get_index(x, y, z) as usize] = block;
    }
}