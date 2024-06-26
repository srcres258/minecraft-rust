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

pub type BlockType = u8;

/// @brief Known block ID types used in game.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BlockId {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
    OakBark = 4,
    OakLeaf = 5,
    Sand = 6,
    Water = 7,
    Cactus = 8,
    Rose = 9,
    TallGrass = 10,
    DeadShrub = 11
}

impl BlockId {
    pub const NUM_TYPES: usize = 12;
}

impl TryFrom<i32> for BlockId {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == BlockId::Air as i32 => Ok(BlockId::Air),
            x if x == BlockId::Grass as i32 => Ok(BlockId::Grass),
            x if x == BlockId::Dirt as i32 => Ok(BlockId::Dirt),
            x if x == BlockId::Stone as i32 => Ok(BlockId::Stone),
            x if x == BlockId::OakBark as i32 => Ok(BlockId::OakBark),
            x if x == BlockId::OakLeaf as i32 => Ok(BlockId::OakLeaf),
            x if x == BlockId::Sand as i32 => Ok(BlockId::Sand),
            x if x == BlockId::Water as i32 => Ok(BlockId::Water),
            x if x == BlockId::Cactus as i32 => Ok(BlockId::Cactus),
            x if x == BlockId::Rose as i32 => Ok(BlockId::Rose),
            x if x == BlockId::TallGrass as i32 => Ok(BlockId::TallGrass),
            x if x == BlockId::DeadShrub as i32 => Ok(BlockId::DeadShrub),
            _ => Err(())
        }
    }
}

impl Default for BlockId {
    fn default() -> Self {
        Self::Air
    }
}