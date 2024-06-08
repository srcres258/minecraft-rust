extern crate nalgebra_glm as glm;

use sfml::window::mouse::Button;
use crate::item::material::Material;
use crate::player::player::Player;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::event::world_event::IWorldEvent;
use crate::world::world::World;

/// @brief Event class that handles what happens to a block in a world as a player interacts.
pub struct PlayerDigEvent<'a> {
    button_press: Button,
    dig_spot: glm::TVec3<f32>,
    p_player: &'a Player<'a>
}

impl<'a> PlayerDigEvent<'a> {
    pub fn new(
        button: Button,
        location: glm::TVec3<f32>,
        player: &'a Player<'_>
    ) -> Self {
        Self {
            button_press: button,
            dig_spot: location,
            p_player: player
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
                //todo
                world.update_chunk(x, y, z);
                world.set_block(x, y, z, ChunkBlock::new_with_block_type(0));
            }
            Button::Right => {
                //todo
            }
            _ => {}
        }
    }
}

impl<'a> IWorldEvent for PlayerDigEvent<'a> {
    fn handle(&self, world: &mut World) {
        let chunk_location = World::get_chunk_xz(
            self.dig_spot.x as i32, self.dig_spot.z as i32);

        if world.get_chunk_manager().chunk_loaded_at(chunk_location.x, chunk_location.z) {
            //todo
        }
    }
}