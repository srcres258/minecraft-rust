extern crate nalgebra_glm as glm;

/// @brief Collision detection class for 3D environment.
pub struct AABB {
    position: glm::TVec3<f32>,
    dimensions: glm::TVec3<f32>
}