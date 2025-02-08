use crate::event::world_event::IWorldEvent;
use crate::item::material::{Material, ID};
use crate::player::player::Player;
use crate::util::mem::UWRefMut;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::world::World;
use nalgebra_glm::Vec3;
use sfml::window::mouse::Button;

pub struct PlayerDigEvent {
    button_press: Button,
    dig_spot: Vec3,
    player: UWRefMut<Player>
}

impl PlayerDigEvent {
    pub fn new(button_press: Button, dig_spot: Vec3, player: UWRefMut<Player>) -> Self {
        Self { button_press, dig_spot, player }
    }

    fn dig(&mut self, world: &mut World) {
        let x = self.dig_spot.x as i32;
        let y = self.dig_spot.y as i32;
        let z = self.dig_spot.z as i32;
        match self.button_press {
            Button::Left => {
                let block = world.block(x, y, z);
                let material = Material::to_material(BlockId::try_from(block.id).unwrap());
                self.player.add_item(material);
                world.update_chunk(x, y, z);
                world.set_block(x, y, z, ChunkBlock::from_block_id(BlockId::Air));
            }
            Button::Right => {
                let stack = self.player.held_items_mut();
                let material = stack.material();

                if material.id != ID::Nothing {
                    stack.remove();
                    world.update_chunk(x, y, z);
                    world.set_block(x, y, z, ChunkBlock::from_block_t(material.id as _));
                }
            }
            _ => {}
        }
    }
}

impl IWorldEvent for PlayerDigEvent {
    fn handle(&mut self, world: &mut World) {
        let chunk_location = World::chunk_xz(self.dig_spot.x as _, self.dig_spot.z as _);
        
        if world.chunk_manager().chunk_loaded_at(chunk_location.x, chunk_location.z) {
            self.dig(world);
        }
    }
}