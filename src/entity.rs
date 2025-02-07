use nalgebra_glm::Vec3;
use crate::physics::aabb::AABB;

pub trait Entity {
    fn position(&self) -> Vec3;
    fn position_mut(&mut self) -> &mut Vec3;
    fn rotation(&self) -> Vec3;
    fn rotation_mut(&mut self) -> &mut Vec3;
    fn velocity(&self) -> Vec3;
    fn velocity_mut(&mut self) -> &mut Vec3;
    fn box_aabb(&self) -> AABB;
    fn box_aabb_mut(&mut self) -> &mut AABB;
}

#[derive(Copy, Clone)]
pub struct EntityImpl {
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    
    pub box_aabb: AABB
}

impl EntityImpl {
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

impl Default for EntityImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for EntityImpl {
    fn position(&self) -> Vec3 {
        self.position
    }
    fn position_mut(&mut self) -> &mut Vec3 {
        &mut self.position
    }
    fn rotation(&self) -> Vec3 {
        self.rotation
    }
    fn rotation_mut(&mut self) -> &mut Vec3 {
        &mut self.rotation
    }
    fn velocity(&self) -> Vec3 {
        self.velocity
    }
    fn velocity_mut(&mut self) -> &mut Vec3 {
        &mut self.velocity
    }
    fn box_aabb(&self) -> AABB {
        self.box_aabb
    }
    fn box_aabb_mut(&mut self) -> &mut AABB {
        &mut self.box_aabb
    }
}