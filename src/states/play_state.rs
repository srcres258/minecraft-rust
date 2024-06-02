use crate::application::Application;

pub struct StatePlay<'a, 'b> {
    application: &'a Application<'b>,
    keyboard: Keyboard,
    player: Player,
    world: World,
    fps_counter: FPSCounter
}