use crate::item::material;
use crate::item::material::Material;

/// @brief Determines if a player character is holding blocks or items, also determines placement behavior.
pub struct ItemStack {
    p_material: &'static Material,
    num_in_stack: i32
}

impl ItemStack {
    pub fn new(material: &'static Material, amount: i32) -> Self {
        Self {
            p_material: material,
            num_in_stack: amount
        }
    }

    pub fn add(&mut self, amount: i32) -> i32 {
        self.num_in_stack += amount;

        if self.num_in_stack > self.p_material.max_stack_size {
            let left_over = self.num_in_stack - self.p_material.max_stack_size;
            self.num_in_stack = self.p_material.max_stack_size;
            left_over
        } else {
            0
        }
    }

    pub fn remove(&mut self) {
        self.num_in_stack -= 1;
        if self.num_in_stack == 0 {
            self.p_material = &material::NOTHING;
        }
    }
    
    pub fn num_in_stack(&self) -> i32 {
        self.num_in_stack
    }
    
    pub fn material(&self) -> &'static Material {
        self.p_material
    }
}