use sfml::window::Window;
use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::play_state::StatePlay;
use crate::states::state_base::StateBase;
use crate::world::block::block_database::BlockDatabase;

pub struct Application {
    states: Vec<Box<dyn StateBase>>,

    context: Context,
    master_renderer: RenderMaster,
    camera: Camera,

    is_pop_state: bool
}

impl Application {
    pub fn new(config: Config) -> Self {
        let mut result = Self {
            states: Vec::new(),
            context: Context::new(config),
            master_renderer: , //todo
            camera: ,//todo
            is_pop_state: false
        }

        BlockDatabase::get();
        result.push_state(Box::new(StatePlay::new(&result, config)));

        result
    }

    /// @brief Game loop utilizing a mixture of SFML events and GL rendering.
    pub fn run_loop(&mut self) {
        //todo
    }

    pub fn push_state(&mut self, state: Box<dyn StateBase>) {
        self.states.push(state);
        let s = self.states.last_mut().unwrap();
        s.on_open();
    }

    pub fn pop_state(&mut self) {
        self.is_pop_state = true;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn window(&self) -> &Window {
        &self.context.window
    }
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.context.window
    }

    pub fn turn_off_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(false);
    }
    pub fn turn_on_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(true);
    }

    fn handle_events(&mut self) {
        //todo
    }
}