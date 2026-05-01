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

use nalgebra_glm as glm;

use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use sfml::system::Clock;
use sfml::window::{Event, Key};
use sfml::window::mouse::Button;
use crate::application::Application;
use crate::config::Config;
use crate::input::keyboard::Keyboard;
use crate::input::toggle_key::ToggleKey;
use crate::maths::ray::Ray;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;
use crate::util::fps_counter::FPSCounter;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::block::block_id::BlockId;
use crate::world::event::player_dig_event::PlayerDigEvent;
use crate::world::world::World;

/// Active game playing state, not associated with game menus.
pub struct StatePlay {
    application: Rc<UnsafeCell<Application>>,
    
    keyboard: Keyboard,
    player: Player,
    world: Option<Arc<UnsafeCellWrapper<World>>>,
    
    fps_counter: FPSCounter
}

static DRAW_GUI: AtomicBool = AtomicBool::new(false);

thread_local! {
    static TIMER: RefCell<Option<sfml::SfBox<Clock>>> = RefCell::new(None);
    static DT: RefCell<Option<sfml::SfBox<Clock>>> = RefCell::new(None);
    static DRAW_KEY: RefCell<ToggleKey> = RefCell::new(ToggleKey::new(Key::F3));
}

impl StatePlay {
    pub fn new_boxed(application: Rc<UnsafeCell<Application>>, config: Config) -> Box<Self> {
        let result = Self {
            application: Rc::clone(&application),
            keyboard: Keyboard::new(),
            player: Player::default(),
            world: None,
            fps_counter: FPSCounter::new()
        };
        let mut result = Box::new(result);
        unsafe {
            result.world = Some(World::new(
                (*application.get()).camera(),
                &config,
                &mut result.player)
            );

            (*(*application.get()).camera().get()).update_from_entity(&result.player.base);
        }

        result
    }
}

impl StateBase for StatePlay {
    fn handle_event(&mut self, event: Event) {
        self.keyboard.update(event);
    }

    fn handle_input(&mut self) {
        unsafe {
            self.player.handle_input((*self.application.get()).window_mut(), &self.keyboard);
        }

        let mut last_position: glm::TVec3<f32> = Default::default();

        // Ray is cast as player's 'vision'
        let mut ray = Ray::new(
            &glm::vec3(
                self.player.position.x,
                self.player.position.y + 0.6,
                self.player.position.z
            ),
            &self.player.rotation
        ); // Corrected for camera offset
        while ray.length() < 6. {
            let x = ray.end().x as i32;
            let y = ray.end().y as i32;
            let z = ray.end().z as i32;

            let block = unsafe {
                (*self.world.as_ref().unwrap().get()).get_block(x, y, z)
            };
            let id = BlockId::try_from(block.id.0 as i32).unwrap();

            if id != BlockId::Air && id != BlockId::Water {
                let should_stop = TIMER.with(|timer| {
                    let mut timer = timer.borrow_mut();
                    let timer = timer.get_or_insert_with(Clock::start);

                    if timer.elapsed_time().as_seconds() <= 0.2 {
                        return false;
                    }

                    if Button::Left.is_pressed() {
                        timer.restart();
                        unsafe {
                            (*self.world.as_ref().unwrap().get()).add_event(Box::new(
                                PlayerDigEvent::new(
                                    Button::Left,
                                    ray.end(),
                                    &mut self.player
                                )
                            ));
                        }
                        true
                    } else if Button::Right.is_pressed() {
                        timer.restart();
                        unsafe {
                            (*self.world.as_ref().unwrap().get()).add_event(Box::new(
                                PlayerDigEvent::new(
                                    Button::Right,
                                    last_position,
                                    &mut self.player
                                )
                            ));
                        }
                        true
                    } else {
                        false
                    }
                });

                if should_stop {
                    break;
                }
            }
            last_position = ray.end();

            ray.step(0.05);
        }
    }

    fn update(&mut self, delta_time: f32) {
        if self.player.position.x < 0. {
            self.player.position.x = 0.;
        }
        if self.player.position.z < 0. {
            self.player.position.z = 0.;
        }

        self.fps_counter.update();
        self.player.update(
            delta_time,
            unsafe {
                &mut *self.world.as_ref().unwrap().get()
            }
        );
        // Sync player position to camera
        unsafe {
            let cam = &mut *(*self.application.get()).camera().get();
            cam.update_from_entity(&self.player.base);
        }
        unsafe {
            let arc = Arc::clone(&(*self.application.get()).camera());
            let camera = &*arc.get();
            World::update(&Arc::clone(self.world.as_ref().unwrap()), &camera);
        }
    }

    fn render(&mut self, renderer: &mut RenderMaster) {
        DT.with(|dt| {
            dt.borrow_mut().get_or_insert_with(Clock::start);
        });

        if DRAW_KEY.with(|draw_key| draw_key.borrow_mut().is_key_pressed()) {
            DRAW_GUI.fetch_xor(true, Ordering::Relaxed);
        }

        if DRAW_GUI.load(Ordering::Relaxed) {
            self.fps_counter.draw(renderer);
            self.player.draw(renderer);
        }

        unsafe {
            let arc = Arc::clone(&(*self.application.get()).camera());
            let camera = &*arc.get();
            (*self.world.as_ref().unwrap().get()).render_world(renderer, &camera);
        }
    }

    fn on_open(&mut self) {
        unsafe {
            (*self.application.get()).turn_off_mouse();
        }
    }
}
