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

use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world_constants::CHUNK_SIZE;

#[derive(Default)]
pub struct SuperFlatGenerator {
}

impl TerrainGenerator for SuperFlatGenerator {
    fn generate_terrain_for(&mut self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.set_block(x as _, 0, z as _, ChunkBlock::new_with_block_id(BlockId::Stone));
                chunk.set_block(x as _, 1, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 2, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 3, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                chunk.set_block(x as _, 4, z as _, ChunkBlock::new_with_block_id(BlockId::Grass));
            }
        }
    }

    fn get_minimum_spawn_height(&self) -> i32 {
        1
    }
}