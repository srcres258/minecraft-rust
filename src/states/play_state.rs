use std::sync::{Arc, Mutex, RwLock};
use crate::application::Application;
use crate::config::Config;
use crate::event::player_dig_event::PlayerDigEvent;
use crate::input::keyboard::Keyboard;
use crate::input::toggle_key::ToggleKey;
use crate::maths::ray::Ray;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;
use crate::util::mem::{uw_ref, uw_ref_mut, UWRefMut};
use crate::world::block::block_id::BlockId;
use crate::world::world::World;
use nalgebra_glm::Vec3;
use sfml::system::Clock;
use sfml::window::{mouse, Event, Key};
use sfml::SfBox;

/// @brief Active game playing state, not associated with game menus.
pub struct StatePlay {
    application: Arc<Application>,
    
    keyboard: Keyboard,
    player: Player,
    world: Arc<World>,
    
    // Since FPSCounter has not been fully implemented in the C++ code,
    // ignore its implementation at present. (Consider implementing this later.)

    timer: Option<SfBox<Clock>>,
    dt: Option<SfBox<Clock>>,
    draw_gui: bool,
    draw_key: ToggleKey
}

impl StateBase for StatePlay {
    fn handle_event(&mut self, e: Event) {
        self.keyboard.update(e);
    }

    fn handle_input(&mut self) {
        let context_ref = self.application.context();
        let mut context = context_ref.lock().unwrap();
        self.player.handle_input(&mut context.window, &self.keyboard);
        drop(context);
        drop(context_ref);

        if let None = self.timer {
            self.timer = Some(Clock::start());
        }
        let mut last_position = Vec3::default();

        // Ray is cast as player's 'vision'
        let mut ray = Ray::new(
            Vec3::new(
                self.player.position.x,
                self.player.position.y + 0.6, // Corrected for camera offset
                self.player.position.z
            ),
            self.player.rotation
        );
        while ray.length() < 6. {
            let x = ray.end().x as i32;
            let y = ray.end().y as i32;
            let z = ray.end().z as i32;

            let block = self.world.block(x, y, z);
            let id = BlockId::try_from(block.id).unwrap();

            if id != BlockId::Air && id != BlockId::Water {
                if self.timer.as_ref().unwrap().elapsed_time().as_seconds() > 0.2 {
                    if mouse::Button::Left.is_pressed() {
                        self.timer.as_mut().unwrap().restart();
                        // The player "digs" the block up
                        self.world.add_event(Box::new(PlayerDigEvent::new(
                            mouse::Button::Left,
                            ray.end(),
                            uw_ref_mut(&mut self.player)
                        )));
                        break;
                    } else if mouse::Button::Right.is_pressed() {
                        self.timer.as_mut().unwrap().restart();
                        // Block is placed by player
                        self.world.add_event(Box::new(PlayerDigEvent::new(
                            mouse::Button::Right,
                            ray.end(),
                            uw_ref_mut(&mut self.player)
                        )));
                        break;
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

        self.player.update(delta_time, &self.world);
        self.world.update();
    }

    fn render(&mut self, renderer: &mut RenderMaster) {
        if let None = self.dt {
            self.dt = Some(Clock::start());
        }

        if self.draw_key.key_pressed() {
            self.draw_gui = !self.draw_gui;
        }

        if self.draw_gui {
            self.player.draw(renderer);
        }

        self.world.render_world(renderer, self.application.camera());
    }

    fn on_open(&mut self) {
        self.application.turn_off_mouse();
    }
}

impl StatePlay {
    pub fn new(app: Arc<Application>, config: Config) -> Self {
        let mut player = Player::new();
        let mut result = Self {
            application: Arc::clone(&app),
            world: World::new(app.camera(), config, &mut player),
            keyboard: Keyboard::new(),
            player,
            timer: None,
            dt: None,
            draw_gui: false,
            draw_key: ToggleKey::new(Key::F3)
        };
        
        app.camera().lock().unwrap().hook_entity(uw_ref(&result.player));
        
        result
    }
}