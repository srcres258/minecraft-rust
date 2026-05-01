// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::UnsafeCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use sfml::system::{Clock, Time};
use sfml::window::{Event, Key, Window};
use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::play_state::StatePlay;
use crate::states::state_base::StateBase;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::block::block_database::BlockDatabase;

static TIME_ELAPSED_BITS: AtomicU32 = AtomicU32::new(0.0f32.to_bits());

pub fn time_elapsed() -> f32 {
    f32::from_bits(TIME_ELAPSED_BITS.load(Ordering::Relaxed))
}

fn add_time_elapsed(delta: f32) {
    let mut current = TIME_ELAPSED_BITS.load(Ordering::Relaxed);

    loop {
        let next = (f32::from_bits(current) + delta).to_bits();
        match TIME_ELAPSED_BITS.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(actual) => current = actual,
        }
    }
}

/// The main game application itself.
pub struct Application {
    states: Vec<Box<dyn StateBase>>,
    context: Context,
    master_renderer: RenderMaster,
    camera: Arc<UnsafeCellWrapper<Camera>>,
    #[allow(dead_code)]
    config: Config,
    is_pop_state: bool
}

impl Application {
    pub fn new(config: Config) -> Rc<UnsafeCell<Self>> {
        let result = Self {
            states: Vec::new(),
            context: Context::new(config),
            master_renderer: RenderMaster::default(),
            camera: Arc::new(UnsafeCellWrapper::new(Camera::new(config))),
            config,
            is_pop_state: false
        };
        let result = Rc::new(UnsafeCell::new(result));

        BlockDatabase::get();
        // SAFETY: Application is being initialized on a single thread. No other references exist yet.
        unsafe {
            (*result.get()).push_state(StatePlay::new_boxed(Rc::clone(&result), config));
        }

        result
    }

    /// Game loop utilizing a mixture of SFML events and GL rendering.
    pub fn run_loop(&mut self) {
        let mut dt_timer = Clock::start();
        let mut dt = Clock::start();

        #[allow(unused_assignments)]
        let mut m = Time::default();

        while self.context.window.is_open() && !self.states.is_empty() {
            let delta_time = dt_timer.restart();
            let state = self.states.last_mut().unwrap();

            state.handle_input();
            state.update(delta_time.as_seconds());
            // SAFETY: Camera is accessed from the main thread only. The Arc<UnsafeCellWrapper<Camera>> pattern is safe because camera is only mutated during the main game loop where no concurrent access occurs.
            unsafe {
                (*self.camera.get()).update();
            }

            state.render(&mut self.master_renderer);
            // SAFETY: Camera is read from the main thread only. Mutable access (update) has already completed above.
            unsafe {
                self.master_renderer.finish_render(&mut self.context.window, &*self.camera.get());
            }

            self.handle_events();
            if self.is_pop_state {
                self.is_pop_state = false;
                self.states.pop();
            }

            m = dt.restart();

            add_time_elapsed(m.as_seconds());
        }
    }

    /// Handles window events, especially window polling and keyboard inputs.
    pub fn push_state(&mut self, state: Box<dyn StateBase>) {
        self.states.push(state);
        let s = self.states.last_mut().unwrap();
        s.on_open();
    }

    /// Tell the program stack to pop off the state.
    pub fn pop_state(&mut self) {
        self.is_pop_state = true;
    }

    pub fn camera(&self) -> Arc<UnsafeCellWrapper<Camera>> {
        Arc::clone(&self.camera)
    }

    pub fn window(&self) -> &Window {
        &self.context.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.context.window
    }

    /// Makes the mouse invisible, doesn't actually turn off the mouse
    pub fn turn_off_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(false);
    }

    /// Makes the mouse visible again.
    pub fn turn_on_mouse(&mut self) {
        self.context.window.set_mouse_cursor_visible(true);
    }

    /// Handles window events, especially window polling and keyboard inputs.
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
