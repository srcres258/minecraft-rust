extern crate nalgebra_glm as glm;

use std::ptr;
use sfml::graphics::{Color, Font, Text, Transformable};
use sfml::SfBox;
use sfml::system::Vector2f;
use sfml::window::{Key, Window};
use crate::entity::Entity;
use crate::input::keyboard::Keyboard;
use crate::input::toggle_key::ToggleKey;
use crate::item::item_stack::ItemStack;
use crate::item::material;
use crate::item::material::Material;
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

impl<'a> Player<'a> {
    pub fn handle_input(&mut self, window: &Window, keyboard: &Keyboard) {
        //todo
    }

    pub fn update(&mut self, world: &World) {
        //todo
    }

    pub fn collide(&mut self, vel: &glm::TVec3<f32>, dt: f32) {
        //todo
    }

    pub fn add_item(&mut self, material: &Material) {
        //todo
    }

    pub fn draw(&mut self, master: &RenderMaster) {
        //todo
    }

    pub fn get_held_items(&self) -> &ItemStack {
        //todo
    }

    pub fn get_held_items_mut(&mut self) -> &mut ItemStack {
        //todo
    }

    fn jump(&mut self) {
        //todo
    }

    fn keyboard_input(&mut self, keyboard: &Keyboard) {
        //todo
    }

    fn mouse_input(&mut self, window: &Window) {
        //todo
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