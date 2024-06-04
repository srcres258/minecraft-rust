use std::cell::RefCell;
use std::rc::Rc;
use crate::application::Application;
use crate::input::keyboard::Keyboard;
use crate::player::player::Player;
use crate::util::fps_counter::FPSCounter;
use crate::world::world::World;

pub struct StatePlay<'a> {
    application: Rc<RefCell<Application>>,
    
    keyboard: Keyboard,
    player: Player<'a>,
    world: World,
    
    fps_counter: FPSCounter<'a>
}