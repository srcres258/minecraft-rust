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

#[derive(Copy, Clone, Default)]
pub struct Block {
    id: BlockId,
    x: i32,
    y: i32,
    z: i32
}

#[derive(Clone, Default)]
pub struct StructureBuilder {
    blocks: Vec<Block>
}

impl Block {
    pub fn new(id: BlockId, x: i32, y: i32, z: i32) -> Self {
        Self { id, x, y, z }
    }
}

impl StructureBuilder {
    pub fn build(&self, chunk: &mut Chunk) {
        for block in self.blocks.iter() {
            chunk.set_block(block.x, block.y, block.z,
                            ChunkBlock::new_with_block_id(block.id));
        }
    }

    pub fn make_column(
        &mut self,
        x: i32,
        z: i32,
        y_start: i32,
        height: i32,
        block: BlockId
    ) {
        for y in y_start..y_start + height {
            self.add_block(x, y, z, block);
        }
    }

    pub fn make_row_x(
        &mut self,
        x_start: i32,
        x_end: i32,
        y: i32,
        z: i32,
        block: BlockId
    ) {
        for x in x_start..=x_end {
            self.add_block(x, y, z, block);
        }
    }

    pub fn make_row_z(
        &mut self,
        z_start: i32,
        z_end: i32,
        x: i32,
        y: i32,
        block: BlockId
    ) {
        for z in z_start..=z_end {
            self.add_block(x, y, z, block);
        }
    }

    pub fn fill(
        &mut self,
        y: i32,
        x_start: i32,
        x_end: i32,
        z_start: i32,
        z_end: i32,
        block: BlockId
    ) {
        for x in x_start..x_end {
            for z in z_start..z_end {
                self.add_block(x, y, z, block);
            }
        }
    }

    pub fn add_block(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        block: BlockId
    ) {
        self.blocks.push(Block::new(block, x, y, z));
    }
}