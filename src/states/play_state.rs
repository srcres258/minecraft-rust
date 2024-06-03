use crate::application::Application;
use crate::input::keyboard::Keyboard;
use crate::player::player::Player;
use crate::world::world::World;

pub struct StatePlay<'a, 'b> {
    application: &'b Application<'a>,
    keyboard: Keyboard,
    player: Player<'a>,
    world: World,
    fps_counter: FPSCounter
}