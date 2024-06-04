extern crate nalgebra_glm as glm;

use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::Entity;

pub struct Camera {
    pub wrapped_obj: Entity,
    
    p_entity: Option<Rc<RefCell<Entity>>>,

    frustum: ViewFrustum,

    projection_matrix: glm::TMat4<f32>,
    view_matrix: glm::TMat4<f32>,
    proj_view_matrix: glm::TMat4<f32>,
    
    config: Config
}