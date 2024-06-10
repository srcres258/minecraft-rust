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

use std::ffi::c_void;
use sfml::window::mouse::Button;
use crate::item::material::{ID, Material};
use crate::player::player::Player;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::event::world_event::IWorldEvent;
use crate::world::world::World;

pub struct PtrMutPlayer(*mut c_void);

/// @brief Event class that handles what happens to a block in a world as a player interacts.
pub struct PlayerDigEvent {
    button_press: Button,
    dig_spot: glm::TVec3<f32>,
    p_player: PtrMutPlayer
}

impl PtrMutPlayer {
    pub fn as_mut(&self) -> &mut Player<'static> {
        unsafe { &mut *(self.0 as *mut Player<'static>) }
    }
}

impl AsRef<Player<'static>> for PtrMutPlayer {
    fn as_ref(&self) -> &Player<'static> {
        unsafe { &*(self.0 as *mut Player<'static>) }
    }
}

unsafe impl Send for PtrMutPlayer {}

impl PlayerDigEvent {
    pub fn new(
        button: Button,
        location: glm::TVec3<f32>,
        player: *mut Player
    ) -> Self {
        Self {
            button_press: button,
            dig_spot: location,
            p_player: PtrMutPlayer(player as _)
        }
    }

    fn dig(&self, world: &mut World) {
        let x = self.dig_spot.x as i32;
        let y = self.dig_spot.y as i32;
        let z = self.dig_spot.z as i32;
        match self.button_press {
            Button::Left => {
                let block = world.get_block(x, y, z);
                let material = Material::from_block_id(BlockId::try_from(
                    block.id as i32).unwrap());
                self.p_player.as_mut().add_item(material);
                world.update_chunk(x, y, z);
                world.set_block(x, y, z, ChunkBlock::new_with_block_type(0));
            }
            Button::Right => {
                let stack = self.p_player.as_mut().get_held_items_mut();
                let material = stack.material();

                if material.id == ID::Nothing {
                    return;
                } else {
                    stack.remove();
                    world.update_chunk(x, y, z);
                    world.set_block(x, y, z, ChunkBlock::new_with_block_id(material.to_block_id()));
                }
            }
            _ => {}
        }
    }
}

impl IWorldEvent for PlayerDigEvent {
    fn handle(&self, world: &mut World) {
        let chunk_location = World::get_chunk_xz(
            self.dig_spot.x as i32, self.dig_spot.z as i32);

        if world.get_chunk_manager().chunk_loaded_at(chunk_location.x, chunk_location.z) {
            self.dig(world);
        }
    }
}