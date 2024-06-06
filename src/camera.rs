extern crate nalgebra_glm as glm;

use std::cell::RefCell;
use std::rc::Rc;
use crate::config::Config;
use crate::entity::Entity;
use crate::maths::frustum::ViewFrustum;
use crate::maths::matrix;

pub struct Camera {
    pub wrapped_obj: Rc<RefCell<Entity>>,
    
    p_entity: Option<Rc<RefCell<Entity>>>,

    frustum: ViewFrustum,

    projection_matrix: glm::TMat4<f32>,
    view_matrix: glm::TMat4<f32>,
    proj_view_matrix: glm::TMat4<f32>,
    
    config: Config
}

impl Camera {
    pub fn new(config: Config) -> Self {
        let mut obj = Entity::default();
        obj.position = glm::vec3(0., 0., -3.5);
        let projection_matrix = matrix::make_projection_matrix(&config);

        Self {
            wrapped_obj: Rc::new(RefCell::new(obj)),
            p_entity: None,
            frustum: ViewFrustum::default(),
            projection_matrix,
            view_matrix: Default::default(),
            proj_view_matrix: Default::default(),
            config
        }
    }

    pub fn update(&mut self) {
        let mut wrapped_obj = self.wrapped_obj.get_mut();
        let p_entity = self.p_entity.clone().unwrap().borrow();
        wrapped_obj.position = glm::vec3(p_entity.position.x, p_entity.position.y + 0.6, p_entity.position.z);
        wrapped_obj.rotation = p_entity.rotation;

        self.view_matrix = matrix::make_view_matrix(self);
        self.proj_view_matrix = self.projection_matrix * self.view_matrix;
        self.frustum.update(&self.proj_view_matrix);
    }

    pub fn hook_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.p_entity = Some(entity);
    }

    pub fn get_view_matrix(&self) -> glm::TMat4<f32> {
        self.view_matrix
    }

    pub fn get_proj_matrix(&self) -> glm::TMat4<f32> {
        self.projection_matrix
    }

    pub fn get_projection_view_matrix(&self) -> glm::TMat4<f32> {
        self.proj_view_matrix
    }

    pub fn get_frustum(&self) -> &ViewFrustum {
        &self.frustum
    }
}