extern crate nalgebra_glm as glm;

use crate::physics::aabb::AABB;

pub struct Entity {
    position: glm::TVec3<f32>,
    rotation: glm::TVec3<f32>,
    velocity: glm::TVec3<f32>,

    box_aabb: AABB
}