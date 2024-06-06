extern crate nalgebra_glm as glm;

use std::cell::RefCell;
use std::rc::Rc;
use sfml::graphics::Text;
use crate::entity::Entity;

pub struct Player<'a> {
    pub wrapped_obj: Rc<RefCell<Entity>>,

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