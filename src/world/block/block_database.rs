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

use std::ptr;
use std::sync::{Arc, RwLock};
use crate::texture::texture_atlas::TextureAtlas;
use crate::world::block::block_data::BlockData;
use crate::world::block::block_id::BlockId;
use crate::world::block::block_types::block_type::{BlockType, DefaultBlock};

static mut INSTANCE_PTR: *mut BlockDatabase = ptr::null_mut();

/// @brief Singleton class that determines status and ID of blocks as a whole.
pub struct BlockDatabase {
    pub texture_atlas: TextureAtlas,
    blocks: [Box<dyn BlockType>; BlockId::NUM_TYPES]
}

impl BlockDatabase {
    fn new() -> Self {
        let texture_atlas = TextureAtlas::new("DefaultPack");
        let blocks: [Box<dyn BlockType>; 12] = [
            Box::new(DefaultBlock::new("Air")),
            Box::new(DefaultBlock::new("Grass")),
            Box::new(DefaultBlock::new("Dirt")),
            Box::new(DefaultBlock::new("Stone")),
            Box::new(DefaultBlock::new("OakBark")),
            Box::new(DefaultBlock::new("OakLeaf")),
            Box::new(DefaultBlock::new("Sand")),
            Box::new(DefaultBlock::new("Water")),
            Box::new(DefaultBlock::new("Cactus")),
            Box::new(DefaultBlock::new("TallGrass")),
            Box::new(DefaultBlock::new("Rose")),
            Box::new(DefaultBlock::new("DeadShrub"))
        ];
        Self { texture_atlas, blocks }
    }

    pub fn get() -> &'static Self {
        unsafe {
            if INSTANCE_PTR == ptr::null_mut() {
                // Allocate the instance on heap memory,
                // then leak it to get the raw pointer.
                let instance = Box::new(BlockDatabase::new());
                INSTANCE_PTR = Box::leak(instance);
            }

            &*INSTANCE_PTR
        }
    }

    pub fn get_block(&self, id: BlockId) -> &dyn BlockType {
        self.blocks[id as usize].as_ref()
    }

    pub fn get_data(&self, id: BlockId) -> Arc<RwLock<BlockData>> {
        self.blocks[id as usize].data()
    }
}