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

extern crate nalgebra_glm as glm;

use std::ops::{Deref, DerefMut};
use std::ptr;
use sfml::graphics::{Color, Font, Text, Transformable};
use sfml::SfBox;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::{Key, Window};
use crate::entity::Entity;
use crate::input::keyboard::Keyboard;
use crate::input::toggle_key::ToggleKey;
use crate::item::item_stack::ItemStack;
use crate::item::material;
use crate::item::material::{ID, Material};
use crate::renderer::render_master::RenderMaster;
use crate::world::world::World;

pub struct Player<'a> {
    pub base: Entity,

    is_on_ground: bool,
    is_flying: bool,
    is_sneak: bool,

    items: Vec<ItemStack>,
    item_text: Vec<Text<'a>>,
    pos_print: Text<'a>,
    held_item: i32,

    item_down: ToggleKey,
    item_up: ToggleKey,
    fly_key: ToggleKey,

    num1: ToggleKey,
    num2: ToggleKey,
    num3: ToggleKey,
    num4: ToggleKey,
    num5: ToggleKey,

    slow: ToggleKey,

    acceleration: glm::TVec3<f32>
}

static mut FONT: *mut SfBox<Font> = ptr::null_mut();
static mut USE_MOUSE: bool = true;
static mut USE_MOUSE_KEY_PTR: *mut ToggleKey = ptr::null_mut();
static mut LAST_MOUSE_POSITION_PTR: *mut Vector2i = ptr::null_mut();

const SPEED: f32 = 0.2;

impl<'a> Player<'a> {
    pub fn handle_input(&mut self, window: &mut Window, keyboard: &Keyboard) {
        self.keyboard_input(keyboard);
        self.mouse_input(window);

        if self.item_down.is_key_pressed() {
            self.held_item += 1;
            if self.held_item == self.items.len() as i32 {
                self.held_item = 0;
            }
        } else if self.item_up.is_key_pressed() {
            self.held_item -= 1;
            if self.held_item == -1 {
                self.held_item = self.items.len() as i32 - 1;
            }
        }

        if self.fly_key.is_key_pressed() {
            self.is_flying = !self.is_flying;
        }

        if self.num1.is_key_pressed() {
            self.held_item = 0;
        }
        if self.num2.is_key_pressed() {
            self.held_item = 1;
        }
        if self.num3.is_key_pressed() {
            self.held_item = 2;
        }
        if self.num4.is_key_pressed() {
            self.held_item = 3;
        }
        if self.num5.is_key_pressed() {
            self.held_item = 4;
        }
        if self.slow.is_key_pressed() {
            self.is_sneak = !self.is_sneak;
        }
    }

    pub fn update(&mut self, dt: f32, world: &mut World) {
        self.base.velocity += self.acceleration;
        self.acceleration = glm::vec3(0., 0., 0.);

        if !self.is_flying {
            if !self.is_on_ground {
                self.base.velocity.y -= 40. * dt;
            }
            self.is_on_ground = false;
        }

        if self.base.position.y <= 0. && !self.is_flying {
            self.base.position.y = 300.;
        }

        self.base.position.x += self.base.velocity.x * dt;
        self.collide(world, &glm::vec3(self.base.velocity.x, 0., 0.), dt);

        self.base.position.y += self.base.velocity.y * dt;
        self.collide(world, &glm::vec3(0., self.base.velocity.y, 0.), dt);

        self.base.position.z += self.base.velocity.z * dt;
        self.collide(world, &glm::vec3(0., 0., self.base.velocity.z), dt);

        self.base.box_aabb.update(&self.base.position);
        self.base.velocity.x *= 0.95;
        self.base.velocity.z *= 0.95;
        if self.is_flying {
            self.base.velocity.y *= 0.95;
        }
    }

    pub fn collide(&mut self, world: &mut World, vel: &glm::TVec3<f32>, _dt: f32) {
        let mut x = (self.base.position.x - self.base.box_aabb.dimensions.x) as i32;
        while (x as f32) < self.base.position.x + self.base.box_aabb.dimensions.x {
            let mut y = (self.base.position.y - self.base.box_aabb.dimensions.y) as i32;
            while (y as f32) < self.base.position.y + 0.7 {
                let mut z = (self.base.position.z - self.base.box_aabb.dimensions.z) as i32;
                while (z as f32) < self.base.position.z + self.base.box_aabb.dimensions.z {
                    let block = world.get_block(x, y, z);

                    if block.id != 0 && block.get_data().read().unwrap().block_data().is_collidable {
                        if vel.y > 0. {
                            self.base.position.y = y as f32 - self.base.box_aabb.dimensions.y;
                            self.base.velocity.y = 0.;
                        } else if vel.y < 0. {
                            self.is_on_ground = true;
                            self.base.position.y = y as f32 + self.base.box_aabb.dimensions.y + 1.;
                            self.base.velocity.y = 0.;
                        }

                        if vel.x > 0. {
                            self.base.position.x = x as f32 - self.base.box_aabb.dimensions.x;
                        } else if vel.x < 0. {
                            self.base.position.x = x as f32 + self.base.box_aabb.dimensions.x + 1.;
                        }

                        if vel.z > 0. {
                            self.base.position.z = z as f32 - self.base.box_aabb.dimensions.z;
                        } else if vel.z < 0. {
                            self.base.position.z = z as f32 + self.base.box_aabb.dimensions.z + 1.;
                        }
                    }
                    z += 1;
                }
                y += 1;
            }
            x += 1;
        }
    }

    pub fn add_item(&mut self, material: &'static Material) {
        let id = material.id;

        for i in 0..self.items.len() {
            if self.items[i].material().id == id {
                self.items[i].add(1);
                return;
            } else if self.items[i].material().id == ID::Nothing {
                self.items[i] = ItemStack::new(material, 1);
                return;
            }
        }
    }

    pub fn draw(&mut self, _master: &RenderMaster) {
        for i in 0..self.items.len() {
            let t = &mut self.item_text[i];
            if i == self.held_item as usize {
                t.set_fill_color(Color::RED);
            } else {
                t.set_fill_color(Color::WHITE);
            }
            t.set_string(format!(
                "{} {} ",
                self.items[i].material().name,
                self.items[i].num_in_stack()
            ).as_str());
        }
        self.pos_print.set_string(format!(
            " X: {} Y: {} Z: {} Grounded {}",
            self.base.position.x,
            self.base.position.y,
            self.base.position.z,
            self.is_on_ground
        ).as_str());
    }

    pub fn get_held_items(&self) -> &ItemStack {
        &self.items[self.held_item as usize]
    }

    pub fn get_held_items_mut(&mut self) -> &mut ItemStack {
        &mut self.items[self.held_item as usize]
    }

    fn jump(&mut self) {
        if !self.is_flying {
            if self.is_on_ground {
                self.is_on_ground = false;
                self.acceleration.y += SPEED * 50.;
            }
        } else {
            self.acceleration.y += SPEED * 3.;
        }
    }

    fn keyboard_input(&mut self, keyboard: &Keyboard) {
        if keyboard.is_key_down(Key::W) {
            let mut s = SPEED;
            if Key::LControl.is_pressed() {
                s *= 5.;
            } else if Key::RShift.is_pressed() || Key::LShift.is_pressed() {
                s *= 0.35;
            }
            self.acceleration.x += -(self.base.rotation.y + 90.).to_radians().cos() * s;
            self.acceleration.z += -(self.base.rotation.y + 90.).to_radians().sin() * s;
        }
        if keyboard.is_key_down(Key::S) {
            self.acceleration.x += (self.base.rotation.y + 90.).to_radians().cos() * SPEED;
            self.acceleration.z += (self.base.rotation.y + 90.).to_radians().sin() * SPEED;
        }
        if keyboard.is_key_down(Key::A) {
            self.acceleration.x += -self.base.rotation.y.to_radians().cos() * SPEED;
            self.acceleration.z += -self.base.rotation.y.to_radians().sin() * SPEED;
        }
        if keyboard.is_key_down(Key::D) {
            self.acceleration.x += self.base.rotation.y.to_radians().cos() * SPEED;
            self.acceleration.z += self.base.rotation.y.to_radians().sin() * SPEED;
        }

        if keyboard.is_key_down(Key::Space) {
            self.jump();
        } else if keyboard.is_key_down(Key::LShift) && self.is_flying {
            self.acceleration.y -= SPEED * 3.;
        }
    }

    fn mouse_input(&mut self, window: &mut Window) {
        unsafe {
            if USE_MOUSE_KEY_PTR == ptr::null_mut() {
                let use_mouse_key = Box::new(ToggleKey::new(Key::L));
                USE_MOUSE_KEY_PTR = Box::leak(use_mouse_key);
            }

            if (*USE_MOUSE_KEY_PTR).is_key_pressed() {
                USE_MOUSE = !USE_MOUSE;
            }

            if !USE_MOUSE {
                return;
            }
        }

        const BOUND: f32 = 89.;
        unsafe {
            if LAST_MOUSE_POSITION_PTR == ptr::null_mut() {
                let mut last_mouse_position = window.position();
                last_mouse_position += Vector2i::new(window.size().x as i32 / 2, window.size().y as i32 / 2);
                let last_mouse_position = Box::new(last_mouse_position);
                LAST_MOUSE_POSITION_PTR = Box::leak(last_mouse_position);
                window.set_mouse_position(*LAST_MOUSE_POSITION_PTR);
            }
        }
        let change = unsafe { window.mouse_position() - *LAST_MOUSE_POSITION_PTR };

        self.base.rotation.y += change.x as f32 * 0.05;
        self.base.rotation.x += change.y as f32 * 0.05;

        if self.base.rotation.x > BOUND {
            self.base.rotation.x = BOUND;
        } else if self.base.rotation.x < -BOUND {
            self.base.rotation.x = -BOUND;
        }

        if self.base.rotation.y > 360. {
            self.base.rotation.y = 0.;
        } else if self.base.rotation.y < 0. {
            self.base.rotation.y = 360.;
        }

        unsafe {
            window.set_mouse_position(*LAST_MOUSE_POSITION_PTR);
        }
    }
}

impl<'a> Default for Player<'a> {
    fn default() -> Self {
        let mut result = Self {
            base: Entity::new_ex_2(&glm::vec3(2500., 125., 2500.), &glm::vec3(0., 0., 0.), &glm::vec3(0.3, 1., 0.3)),
            is_on_ground: false,
            is_flying: false,
            is_sneak: false,
            items: Vec::new(),
            item_text: Vec::new(),
            pos_print: Text::default(),
            held_item: 0,
            item_down: ToggleKey::new(Key::Down),
            item_up: ToggleKey::new(Key::Up),
            fly_key: ToggleKey::new(Key::F),
            num1: ToggleKey::new(Key::Num1),
            num2: ToggleKey::new(Key::Num2),
            num3: ToggleKey::new(Key::Num3),
            num4: ToggleKey::new(Key::Num4),
            num5: ToggleKey::new(Key::Num5),
            slow: ToggleKey::new(Key::LShift),
            acceleration: glm::vec3(0., 0., 0.)
        };

        unsafe {
            if FONT == ptr::null_mut() {
                let f = Box::new(Font::from_file("Res/Fonts/rs.ttf").unwrap());
                FONT = Box::leak(f);
            }
        }

        for _ in 0..5 {
            result.items.push(ItemStack::new(&material::NOTHING, 0));
        }

        for i in 0..5 {
            let mut t = Text::default();
            unsafe {
                t.set_font(&*FONT);
            }
            t.set_outline_color(Color::BLACK);
            t.set_character_size(25);
            t.set_position(Vector2f::new(20., 20. * i as f32 + 100.));
            result.item_text.push(t);
        }
        unsafe {
            result.pos_print.set_font(&*FONT);
        }
        result.pos_print.set_outline_color(Color::BLACK);
        result.pos_print.set_character_size(25);
        result.pos_print.set_position(Vector2f::new(20., 20. * 6. + 100.));

        result
    }
}

impl<'a> Deref for Player<'a> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> DerefMut for Player<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}