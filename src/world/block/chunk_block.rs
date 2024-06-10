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

use std::sync::{Arc, RwLock};
use crate::world::block::block_data::BlockData;
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::{BlockId, BlockType};

#[derive(Copy, Clone, Default)]
pub struct ChunkBlock {
    pub id: BlockType
}

impl ChunkBlock {
    pub fn new_with_block_type(id: BlockType) -> Self {
        Self { id }
    }

    pub fn new_with_block_id(id: BlockId) -> Self {
        Self { id: id as BlockType }
    }
    
    pub fn get_data(&self) -> Arc<RwLock<BlockData>> {
        BlockDatabase::get()
            .get_data(BlockId::try_from(self.id as i32).unwrap())
    }
}