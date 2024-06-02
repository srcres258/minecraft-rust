extern crate nalgebra_glm as glm;

use crate::entity::Entity;

pub struct Camera<'a> {
    wrapped_obj: Entity,
    
    p_entity: &'a Entity,

    frustum: ViewFrustum,

    projection_matrix: glm::TMat4<f32>,
    view_matrix: glm::TMat4<f32>,
    proj_view_matrix: glm::TMat4<f32>,
    
    config: Config
}