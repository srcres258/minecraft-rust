use sfml::system::{Clock, Time, Vector2i};
use sfml::window::{Event, Key, VideoMode, Window};
use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::play_state::StatePlay;
use crate::states::state_base::StateBase;
use crate::world::block::block_database::BlockDatabase;

pub static mut G_TIME_ELAPSED: f32 = 0.;

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
            master_renderer: RenderMaster::default(),
            camera: Camera::new(config),
            is_pop_state: false
        };

        BlockDatabase::get();
        result.push_state(Box::new(StatePlay::new(&result, config)));

        result
    }

    /// @brief Game loop utilizing a mixture of SFML events and GL rendering.
    pub fn run_loop(&mut self) {
        let mut dt_timer = Clock::start();
        let mut dt = Clock::start();

        let mut m = Time::default();

        // Grab the context window and force it to a certain position.
        // This prevents the window from sticking to the bottom of the visible screen like it does
        // in some Linux distros. Especially Arch.

        let win_center = if self.context.window.size().x <= 640 {
            // If the window is small, use these parameters
            Vector2i::new(
                (VideoMode::desktop_mode().width as f64 / 3.5) as _,
                (VideoMode::desktop_mode().width / 4) as _
            )
        } else {
            // Else force it to the upper-leftgit p
            Vector2i::new(0, 0)
        };

        self.context.window.set_position(win_center);

        while self.context.window.is_open() && !self.states.is_empty() {
            let delta_time = dt_timer.restart();
            let state = self.states.last_mut().unwrap();

            state.handle_input();
            state.update(delta_time.as_seconds());
            self.camera.update();

            state.render(&self.master_renderer);
            self.master_renderer.finish_render(&mut self.context.window, &self.camera);

            self.handle_events();
            if self.is_pop_state {
                self.is_pop_state = false;
                self.states.pop();
            }

            m = dt.restart();

            unsafe {
                G_TIME_ELAPSED += m.as_seconds();
            }
        }
    }

    pub fn push_state(&mut self, state: Box<dyn StateBase>) {
        self.states.push(state);
        let s = self.states.last_mut().unwrap();
        s.on_open();
    }

    /// @brief Tell the program stack to pop off the state.
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

    /// @brief Makes the mouse invisible, doesn't actually turn off the mouse.
    pub fn turn_off_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(false);
    }
    /// @brief Makes the mouse visible again.
    pub fn turn_on_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(true);
    }

    /// @brief Handles window events, especially window polling and keyboard inputs.
    fn handle_events(&mut self) {
        while let Some(e) = self.context.window.poll_event() {
            self.states.last_mut().unwrap().handle_event(e);
            match e {
                Event::Closed => {
                    self.context.window.close();
                }
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => {
                            self.context.window.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}