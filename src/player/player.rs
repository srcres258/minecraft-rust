use crate::entity::{Entity, EntityImpl};
use crate::input::keyboard::Keyboard;
use crate::input::toggle_key::ToggleKey;
use crate::item::item_stack::ItemStack;
use crate::item::material;
use crate::item::material::{Material, ID};
use crate::renderer::render_master::RenderMaster;
use crate::util::mem::uw_ref_mut;
use crate::world::world::World;
use nalgebra_glm::Vec3;
use sfml::system::Vector2i;
use sfml::window::{mouse, Key, Window};
use std::ops::{Deref, DerefMut};
use delegate::delegate;
use crate::physics::aabb::AABB;

struct StaticStorage {
    use_mouse: bool,
    use_mouse_key: ToggleKey,
    bound: f32,
    last_mouse_position: Option<Vector2i>
}

/// @brief Player character, including player movements and world interactions.
pub struct Player {
    base: EntityImpl,

    is_on_ground: bool,
    is_flying: bool,
    is_sneak: bool,

    items: Vec<ItemStack>,
    // `m_itemText` is not used for rendering in the original C++ code,
    // hence ignore it in Rust here. (Consider implementing it in Rust later.)
    // `m_posPrint` is not used for rendering in the original C++ code,
    // hence ignore it in Rust here. (Consider implementing it in Rust later.)
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

    acceleration: Vec3,

    static_storage: StaticStorage
}

// The global variable `f` is not used for rendering in the original C++ code,
// hence ignore it in Rust here. (Consider implementing it in Rust later.)

impl Deref for Player {
    type Target = EntityImpl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

const SPEED: f32 = 0.2;

impl Player {
    pub fn new() -> Self {
        let mut result = Self {
            base: EntityImpl::new_ex_2(
                Vec3::new(2500.0, 125.0, 2500.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.3, 1.0, 0.3)
            ),
            item_down: ToggleKey::new(Key::Down),
            item_up: ToggleKey::new(Key::Up),
            fly_key: ToggleKey::new(Key::F),
            num1: ToggleKey::new(Key::Num1),
            num2: ToggleKey::new(Key::Num2),
            num3: ToggleKey::new(Key::Num3),
            num4: ToggleKey::new(Key::Num4),
            num5: ToggleKey::new(Key::Num5),
            slow: ToggleKey::new(Key::LShift),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            items: Vec::new(),
            held_item: 0,
            is_on_ground: false,
            is_flying: false,
            is_sneak: false,
            static_storage: StaticStorage {
                use_mouse: true,
                use_mouse_key: ToggleKey::new(Key::L),
                bound: 89.0,
                last_mouse_position: Some(Vector2i::default())
            }
        };

        for _ in 0 .. 5 {
            result.items.push(ItemStack::new(&material::NOTHING, 0));
        }

        result
    }

    pub fn handle_input(&mut self, window: &mut Window, keyboard: &Keyboard) {
        self.keyboard_input(keyboard);
        self.mouse_input(window);

        if self.item_down.key_pressed() {
            self.held_item += 1;
            if self.held_item == self.items.len() as i32 {
                self.held_item = 0
            }
        } else if self.item_up.key_pressed() {
            self.held_item -= 1;
            if self.held_item == -1 {
                self.held_item = self.items.len() as i32 - 1;
            }
        }

        if self.fly_key.key_pressed() {
            self.is_flying = !self.is_flying;
        }

        if self.num1.key_pressed() {
            self.held_item = 0;
        }
        if self.num2.key_pressed() {
            self.held_item = 1;
        }
        if self.num3.key_pressed() {
            self.held_item = 2;
        }
        if self.num4.key_pressed() {
            self.held_item = 3;
        }
        if self.num5.key_pressed() {
            self.held_item = 4;
        }
        if self.slow.key_pressed() {
            self.is_sneak = !self.is_sneak;
        }
    }

    pub fn update(&mut self, dt: f32, world: &World) {
        uw_ref_mut(self).velocity += self.acceleration;
        self.acceleration = Vec3::new(0.0, 0.0, 0.0);

        if !self.is_flying {
            if !self.is_on_ground {
                self.velocity.y -= 40.0 * dt;
            }
            self.is_on_ground = false
        }

        if self.position.y <= 0.0 && !self.is_flying {
            self.position.y = 300.0;
        }

        self.position.x += self.velocity.x * dt;
        self.collide(world, Vec3::new(self.velocity.x, 0.0, 0.0), dt);

        self.position.y += self.velocity.y * dt;
        self.collide(world, Vec3::new(0.0, self.velocity.y, 0.0), dt);

        self.velocity.z += self.velocity.z * dt;
        self.collide(world, Vec3::new(0.0, 0.0, self.velocity.z), dt);

        uw_ref_mut(self).box_aabb.update(self.position);
        self.velocity.x *= 0.95;
        self.velocity.z *= 0.95;
        if self.is_flying {
            self.velocity.y *= 0.95;
        }
    }
    pub fn collide(&mut self, world: &World, vel: Vec3, dt: f32) {
        let mut x = (self.position.x - self.box_aabb.dimensions.x) as i32;
        while (x as f32) < self.position.x + self.box_aabb.dimensions.x {
            let mut y = (self.position.y - self.box_aabb.dimensions.y) as i32;
            while (y as f32) < self.position.y + self.box_aabb.dimensions.y {
                let mut z = (self.position.z - self.box_aabb.dimensions.z) as i32;
                while (z as f32) < self.position.z + self.box_aabb.dimensions.z {
                    let block = world.block(x, y, z);
                    
                    if block.id != 0 && block.data().is_collidable {
                        if vel.y > 0. {
                            self.position.y = y as f32 - self.box_aabb.dimensions.y;
                            self.velocity.y = 0.;
                        } else if vel.y < 0. {
                            self.is_on_ground = true;
                            self.position.y = y as f32 + self.box_aabb.dimensions.y + 1.;
                            self.velocity.y = 0.;
                        }
                        
                        if vel.x > 0. {
                            self.position.x = x as f32 - self.box_aabb.dimensions.x;
                        } else if vel.x < 0. {
                            self.position.x = x as f32 + self.box_aabb.dimensions.x + 1.;
                        }

                        if vel.z > 0. {
                            self.position.z = z as f32 - self.box_aabb.dimensions.z;
                        } else if vel.z < 0. {
                            self.position.z = z as f32 + self.box_aabb.dimensions.z + 1.;
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

        for i in 0 .. self.items.len() {
            if self.items[i].material().id == id {
                self.items[i].add(1);
                return;
            } else if self.items[i].material().id == ID::Nothing {
                self.items[i] = ItemStack::new(material, 1);
                return;
            }
        }
    }

    pub fn draw(&self, master: &RenderMaster) {
        log::info!(
            "X: {} Y: {} Z: {} Grounded: {}",
            self.position.x, self.position.y, self.position.z,
            self.is_on_ground
        );
    }

    pub fn held_items(&self) -> &ItemStack {
        &self.items[self.held_item as usize]
    }
    pub fn held_items_mut(&mut self) -> &mut ItemStack {
        &mut self.items[self.held_item as usize]
    }

    fn jump(&mut self) {
        if !self.is_flying {
            if !self.is_on_ground {
                self.is_on_ground = false;
                self.acceleration.y += SPEED * 50.0;
            }
        } else {
            self.acceleration.y += SPEED * 3.0;
        }
    }

    fn keyboard_input(&mut self, keyboard: &Keyboard) {
        if keyboard.is_key_down(Key::W) {
            let mut s = SPEED;
            if Key::LControl.is_pressed() {
                s *= 5.0;
            } else if Key::RShift.is_pressed() || Key::LShift.is_pressed() {
                s *= 0.35;
            }
            self.acceleration.x += -(self.rotation.y + 90.0).to_radians().cos() * s;
            self.acceleration.z += -(self.rotation.y + 90.0).to_radians().sin() * s;
        }
        if keyboard.is_key_down(Key::S) {
            self.acceleration.x += (self.rotation.y + 90.0).to_radians().cos() * SPEED;
            self.acceleration.z += (self.rotation.y + 90.0).to_radians().sin() * SPEED;
        }
        if keyboard.is_key_down(Key::A) {
            self.acceleration.x += -self.rotation.y.to_radians().cos() * SPEED;
            self.acceleration.z += -self.rotation.y.to_radians().sin() * SPEED;
        }
        if keyboard.is_key_down(Key::D) {
            self.acceleration.x += self.rotation.y.to_radians().cos() * SPEED;
            self.acceleration.z += self.rotation.y.to_radians().sin() * SPEED;
        }

        if keyboard.is_key_down(Key::Space) {
            self.jump();
        } else if keyboard.is_key_down(Key::LShift) && self.is_flying {
            self.acceleration.y -= SPEED * 3.0;
        }
    }
    fn mouse_input(&mut self, window: &mut Window) {
        if self.static_storage.use_mouse_key.key_pressed() {
            self.static_storage.use_mouse = !self.static_storage.use_mouse
        }

        if !self.static_storage.use_mouse {
            return;
        }

        if let None = self.static_storage.last_mouse_position {
            self.static_storage.last_mouse_position = Some(mouse::desktop_position());
        }
        let change = mouse::desktop_position() - self.static_storage.last_mouse_position.unwrap();

        self.rotation.y += change.x as f32 * 0.05;
        self.rotation.x += change.y as f32 * 0.05;

        if self.rotation.x > self.static_storage.bound {
            self.rotation.x = self.static_storage.bound;
        } else if self.rotation.x < -self.static_storage.bound {
            self.rotation.x = -self.static_storage.bound
        }

        if self.rotation.y > 360.0 {
            self.rotation.y = 0.0;
        } else if self.rotation.y < 0.0 {
            self.rotation.y = 360.0;
        }

        let cx = (window.size().x / 2) as i32;
        let cy = (window.size().y / 2) as i32;

        window.set_mouse_position(Vector2i::new(cx, cy));

        self.static_storage.last_mouse_position = Some(mouse::desktop_position());
    }
}

impl Entity for Player {
    delegate! {
        to self.base {
            fn position(&self) -> Vec3;
            fn position_mut(&mut self) -> &mut Vec3;
            fn rotation(&self) -> Vec3;
            fn rotation_mut(&mut self) -> &mut Vec3;
            fn velocity(&self) -> Vec3;
            fn velocity_mut(&mut self) -> &mut Vec3;
            fn box_aabb(&self) -> AABB;
            fn box_aabb_mut(&mut self) -> &mut AABB;
        }
    }
}