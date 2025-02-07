use nalgebra_glm::Vec3;
use sfml::window::mouse::Button;
use crate::event::world_event::IWorldEvent;
use crate::player::player::Player;
use crate::util::mem::UWRef;
use crate::world::world::World;

pub struct PlayerDigEvent {
    button_press: Button,
    dig_spot: Vec3,
    player: UWRef<Player>
}

impl PlayerDigEvent {
    pub fn new(button_press: Button, dig_spot: Vec3, player: UWRef<Player>) -> Self {
        Self { button_press, dig_spot, player }
    }

    fn dig(&mut self, world: &World) {
        //todo
    }
}

impl IWorldEvent for PlayerDigEvent {
    fn handle(&mut self, world: &World) {
        todo!()
    }
}