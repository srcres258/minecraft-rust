use nalgebra_glm::Vec3;
use crate::physics::aabb::AABB;

#[derive(Copy, Clone)]
pub struct Entity {
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    
    pub box_aabb: AABB
}

impl Entity {
    pub fn new() -> Self {
        Self {
            box_aabb: AABB::new(Vec3::new(0.0, 0.0, 0.0)),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn new_ex_1(pos: Vec3, rot: Vec3) -> Self {
        Self {
            box_aabb: AABB::new(Vec3::new(0.0, 0.0, 0.0)),
            position: pos,
            rotation: rot,
            velocity: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn new_ex_2(pos: Vec3, rot: Vec3, box_: Vec3) -> Self {
        Self {
            box_aabb: AABB::new(box_),
            position: pos,
            rotation: rot,
            velocity: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}