use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::play_state::StatePlay;
use crate::states::state_base::StateBase;
use crate::world::block::block_database::BlockDatabase;
use sfml::system::{Clock, Time, Vector2i};
use sfml::window::{Event, Key, VideoMode};
use std::sync::{Arc, Mutex, RwLock};

pub static mut G_TIME_ELAPSED: f32 = 0.;

pub struct Application {
    states: Arc<RwLock<Vec<Box<dyn StateBase>>>>,

    context: Arc<Mutex<Context>>,
    master_renderer: Arc<Mutex<RenderMaster>>,
    camera: Arc<Mutex<Camera>>,

    is_pop_state: RwLock<bool>
}

impl Application {
    pub fn new(config: Config) -> Arc<Self> {
        let mut result = Arc::new(Self {
            states: Arc::new(RwLock::new(Vec::new())),
            context: Arc::new(Mutex::new(Context::new(config))),
            master_renderer: Arc::new(Mutex::new(RenderMaster::default())),
            camera: Arc::new(Mutex::new(Camera::new(config))),
            is_pop_state: RwLock::new(false)
        });

        BlockDatabase::get();
        let state = Box::new(StatePlay::new(Arc::clone(&result), config));
        result.push_state(state);

        result
    }

    /// @brief Game loop utilizing a mixture of SFML events and GL rendering.
    pub fn run_loop(&self) {
        let mut dt_timer = Clock::start();
        let mut dt = Clock::start();

        let mut m = Time::default();

        // Grab the context window and force it to a certain position.
        // This prevents the window from sticking to the bottom of the visible screen like it does
        // in some Linux distros. Especially Arch.

        let win_center = if self.context.lock().unwrap().window.size().x <= 640 {
            // If the window is small, use these parameters
            Vector2i::new(
                (VideoMode::desktop_mode().width as f64 / 3.5) as _,
                (VideoMode::desktop_mode().width / 4) as _
            )
        } else {
            // Else force it to the upper-leftgit p
            Vector2i::new(0, 0)
        };

        self.context.lock().unwrap().window.set_position(win_center);

        while self.context.lock().unwrap().window.is_open() && !self.states.read().unwrap().is_empty() {
            let delta_time = dt_timer.restart();
            let mut states = self.states.write().unwrap();
            let state = states.last_mut().unwrap();

            state.handle_input();
            state.update(delta_time.as_seconds());
            self.camera.lock().unwrap().update();

            let mut master_renderer = self.master_renderer.lock().unwrap();
            state.render(&mut master_renderer);
            drop(states);
            master_renderer.finish_render(&mut self.context.lock().unwrap().window, &self.camera.lock().unwrap());
            drop(master_renderer);

            self.handle_events();
            if *self.is_pop_state.read().unwrap() {
                *self.is_pop_state.write().unwrap() = false;
                self.states.write().unwrap().pop();
            }

            m = dt.restart();

            unsafe {
                G_TIME_ELAPSED += m.as_seconds();
            }
        }
    }

    pub fn push_state(&self, state: Box<dyn StateBase>) {
        let mut states = self.states.write().unwrap();
        states.push(state);
        let s = states.last_mut().unwrap();
        s.on_open();
    }

    /// @brief Tell the program stack to pop off the state.
    pub fn pop_state(&self) {
        *self.is_pop_state.write().unwrap() = true;
    }

    pub fn context(&self) -> Arc<Mutex<Context>> {
        Arc::clone(&self.context)
    }
    
    pub fn camera(&self) -> Arc<Mutex<Camera>> {
        Arc::clone(&self.camera)
    }

    /// @brief Makes the mouse invisible, doesn't actually turn off the mouse.
    pub fn turn_off_mouse(&self) {
        let mut context = self.context.lock().unwrap();
        context.window.set_mouse_cursor_visible(false);
    }
    /// @brief Makes the mouse visible again.
    pub fn turn_on_mouse(&self) {
        let mut context = self.context.lock().unwrap();
        context.window.set_mouse_cursor_visible(true);
    }

    /// @brief Handles window events, especially window polling and keyboard inputs.
    fn handle_events(&self) {
        let mut context = self.context.lock().unwrap();
        while let Some(e) = context.window.poll_event() {
            self.states.write().unwrap().last_mut().unwrap().handle_event(e);
            match e {
                Event::Closed => {
                    context.window.close();
                }
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => {
                            context.window.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}