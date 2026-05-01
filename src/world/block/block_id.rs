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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct BlockType(pub u8);

impl From<BlockType> for u8 {
    fn from(b: BlockType) -> Self {
        b.0
    }
}

impl From<u8> for BlockType {
    fn from(n: u8) -> Self {
        Self(n)
    }
}

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
        match value as u8 {
            0 => Ok(BlockId::Air),
            1 => Ok(BlockId::Grass),
            2 => Ok(BlockId::Dirt),
            3 => Ok(BlockId::Stone),
            4 => Ok(BlockId::OakBark),
            5 => Ok(BlockId::OakLeaf),
            6 => Ok(BlockId::Sand),
            7 => Ok(BlockId::Water),
            8 => Ok(BlockId::Cactus),
            9 => Ok(BlockId::Rose),
            10 => Ok(BlockId::TallGrass),
            11 => Ok(BlockId::DeadShrub),
            _ => Err(())
        }
    }
}

impl From<BlockId> for u8 {
    fn from(value: BlockId) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for BlockId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as i32)
    }
}

impl Default for BlockId {
    fn default() -> Self {
        Self::Air
    }
}
