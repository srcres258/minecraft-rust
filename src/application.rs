use sfml::window::Window;
use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;

pub struct Application {
    states: Vec<Box<dyn StateBase>>,

    context: Context,
    master_renderer: RenderMaster,
    camera: Camera,

    is_pop_state: bool
}

impl Application {
    pub fn new(config: &Config) -> Self {
        //todo
    }

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