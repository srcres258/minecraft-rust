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

use std::collections::HashMap;
use std::sync::Arc;
use sfml::system::Vector2i;
use crate::camera::Camera;
use crate::maths::vector2xz::VectorXZ;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::classic_over_world_generator::ClassicOverWorldGenerator;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world::World;

pub type ChunkMap = HashMap<VectorXZ, Chunk>;

/// @brief Dynamic chunk manager that affects chunk and block placement.
pub struct ChunkManager {
    chunks: ChunkMap,
    terrain_generator: Box<dyn TerrainGenerator + Send>,

    world: Arc<UnsafeCellWrapper<World>>
}

impl ChunkManager {
    pub fn new(world: Arc<UnsafeCellWrapper<World>>) -> Self {
        Self {
            chunks: HashMap::new(),
            terrain_generator: Box::new(ClassicOverWorldGenerator::new()),
            world
        }
    }

    pub fn get_chunk(&mut self, x: i32, z: i32) -> &Chunk {
        let key = VectorXZ::new(x, z);
        if !self.chunk_exists_at(x, z) {
            let chunk = Chunk::new(Arc::clone(&self.world), Vector2i::new(x, z));
            self.chunks.insert(key, chunk);
        }

        self.chunks.get(&key).unwrap()
    }

    pub fn get_chunk_mut(&mut self, x: i32, z: i32) -> &mut Chunk {
        let key = VectorXZ::new(x, z);
        if !self.chunk_exists_at(x, z) {
            let chunk = Chunk::new(Arc::clone(&self.world), Vector2i::new(x, z));
            self.chunks.insert(key, chunk);
        }

        self.chunks.get_mut(&key).unwrap()
    }

    pub fn get_chunks(&self) -> &ChunkMap {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunks
    }

    pub fn make_mesh(&mut self, x: i32, z: i32, camera: &Camera) -> bool {
        for nx in -1 ..= 1 {
            for nz in -1 ..= 1 {
                self.load_chunk(x + nx, z + nz);
            }
        }

        self.get_chunk_mut(x, z).make_mesh(camera)
    }

    pub fn chunk_loaded_at(&self, x: i32, z: i32) -> bool {
        if !self.chunk_exists_at(x, z) {
            return false;
        }

        self.chunks.get(&VectorXZ::new(x, z)).unwrap().has_loaded()
    }

    pub fn chunk_exists_at(&self, x: i32, z: i32) -> bool {
        match self.chunks.get(&VectorXZ::new(x, z)) {
            Some(_) => true,
            None => false
        }
    }

    pub fn load_chunk(&mut self, x: i32, z: i32) {
        let ptr = &mut self.terrain_generator as *mut Box<dyn TerrainGenerator + Send>;
        self.get_chunk_mut(x, z).load(unsafe { (*ptr).as_mut() });
    }

    pub fn unload_chunk(&mut self, x: i32, z: i32) {
        //@TODO Save chunk to file ?
        if self.chunk_exists_at(x, z) {
            self.chunks.remove(&VectorXZ::new(x, z));
        }
    }

    pub fn delete_meshes(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.1.delete_meshes();
        }
    }

    pub fn get_terrain_generator(&self) -> &dyn TerrainGenerator {
        self.terrain_generator.as_ref()
    }
}