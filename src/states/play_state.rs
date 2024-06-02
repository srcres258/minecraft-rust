use crate::application::Application;

pub struct StatePlay<'a> {
    application: &'a Application,
    keyboard: Keyboard,
    player: Player,
    world: World,
    fps_counter: FPSCounter
}