extern crate nalgebra_glm as glm;

use sfml::window::mouse::Button;
use crate::player::player::Player;
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
        player: &Player
    ) -> Self {
        Self {
            button_press: button,
            dig_spot: location,
            p_player: player
        }
    }
}

impl<'a> IWorldEvent for PlayerDigEvent<'a> {
    fn handle(&self, world: &World) {
        let chunk_location = todo!();
    }
}