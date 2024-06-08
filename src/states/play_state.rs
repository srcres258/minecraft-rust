extern crate nalgebra_glm as glm;

use std::cell::UnsafeCell;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use sfml::SfBox;
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

/// @brief Active game playing state, not associated with game menus.
pub struct StatePlay<'a> {
    application: Rc<UnsafeCell<Application>>,
    
    keyboard: Keyboard,
    player: Player<'a>,
    world: Option<Arc<UnsafeCellWrapper<World>>>,
    
    fps_counter: FPSCounter<'a>
}

static mut TIMER_PTR: *mut SfBox<Clock> = ptr::null_mut();

static mut DT_PTR: *mut SfBox<Clock> = ptr::null_mut();
static mut DRAW_GUI: bool = false;
static mut DRAW_KEY_PTR: *mut ToggleKey = ptr::null_mut();

impl<'a> StatePlay<'a> {
    pub fn new(application: Rc<UnsafeCell<Application>>, config: Config) -> Self {
        let mut result = Self {
            application: Rc::clone(&application),
            keyboard: Keyboard::new(),
            player: Player::default(),
            world: None,
            fps_counter: FPSCounter::new()
        };
        unsafe {
            result.world = Some(World::new((*application.get()).camera(), &config, &mut result.player));

            (*(*application.get()).camera().get()).hook_entity(&result.player.base);
        }

        result
    }
}

impl<'a> StateBase for StatePlay<'a> {
    fn handle_event(&mut self, event: Event) {
        self.keyboard.update(event);
    }

    fn handle_input(&mut self) {
        unsafe {
            self.player.handle_input((*self.application.get()).window_mut(), &self.keyboard);
        }

        unsafe {
            if TIMER_PTR == ptr::null_mut() {
                let timer = Box::new(Clock::start());
                TIMER_PTR = Box::leak(timer);
            }
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
            let id = BlockId::try_from(block.id as i32).unwrap();

            if id != BlockId::Air && id != BlockId::Water {
                unsafe {
                    if (*TIMER_PTR).elapsed_time().as_seconds() > 0.2 {
                        if Button::Left.is_pressed() {
                            (*TIMER_PTR).restart();
                            // The player "digs" the block up
                            (*self.world.as_ref().unwrap().get()).add_event(Box::new(
                                PlayerDigEvent::new(
                                    Button::Left,
                                    ray.end(),
                                    &mut self.player
                                )
                            ));
                            break;
                        } else if Button::Right.is_pressed() {
                            (*TIMER_PTR).restart();
                            // The player "digs" the block up
                            (*self.world.as_ref().unwrap().get()).add_event(Box::new(
                                PlayerDigEvent::new(
                                    Button::Right,
                                    last_position,
                                    &mut self.player
                                )
                            ));
                            break;
                        }
                    }
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
        unsafe {
            let arc = Arc::clone(&(*self.application.get()).camera());
            let camera = &*arc.get();
            World::update(&Arc::clone(self.world.as_ref().unwrap()), &camera);
        }
    }

    fn render(&mut self, renderer: &mut RenderMaster) {
        unsafe {
            if DT_PTR == ptr::null_mut() {
                let dt = Box::new(Clock::start());
                DT_PTR = Box::leak(dt);
            }
            if DRAW_KEY_PTR == ptr::null_mut() {
                let draw_key = Box::new(ToggleKey::new(Key::F3));
                DRAW_KEY_PTR = Box::leak(draw_key);
            }

            if (*DRAW_KEY_PTR).is_key_pressed() {
                DRAW_GUI = !DRAW_GUI;
            }

            if DRAW_GUI {
                self.fps_counter.draw(renderer);
                self.player.draw(renderer);
            }

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