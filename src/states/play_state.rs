use sfml::window::Event;
use crate::application::Application;
use crate::config::Config;
use crate::input::keyboard::Keyboard;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;
use crate::util::mem::{uw_ref, UWRef};
use crate::world::world::World;

/// @brief Active game playing state, not associated with game menus.
pub struct StatePlay {
    application: UWRef<Application>,
    
    keyboard: Keyboard,
    player: Player,
    world: World,
    
    // Since FPSCounter has not been fully implemented in the C++ code,
    // ignore its implementation at present. (Consider implementing this later.)
}

impl StateBase for StatePlay {
    fn handle_event(&mut self, e: Event) {
        todo!()
    }

    fn handle_input(&mut self) {
        todo!()
    }

    fn update(&mut self, delta_time: f32) {
        todo!()
    }

    fn render(&mut self, renderer: &RenderMaster) {
        todo!()
    }

    fn on_open(&mut self) {
        todo!()
    }
}

impl StatePlay {
    pub fn new(app: &Application, config: Config) -> Self {
        let player = Player::new();
        let result = Self {
            application: uw_ref(app),
            world: World::new(app.camera(), config, &player),
            keyboard: Keyboard::new(),
            player
        };
        
        //todo
        
        result
    }
}