use delegate::delegate;
use crate::config::Config;
use crate::entity::{Entity, EntityImpl};
use crate::maths::frustum::ViewFrustum;
use crate::util::mem::{uw_ref, UWRef};
use nalgebra_glm::{Mat4, Vec3};
use crate::maths::matrix::{make_projection_matrix, make_view_matrix};
use crate::physics::aabb::AABB;

#[derive(Default)]
pub struct Camera {
    base: EntityImpl,

    entity: Option<UWRef<dyn Entity>>,

    frustum: ViewFrustum,

    projection_matrix: Mat4,
    view_matrix: Mat4,
    projection_view_matrix: Mat4,

    config: Config
}

impl Camera {
    pub fn new(config: Config) -> Self {
        let mut result = Self::default();

        result.config = config;
        result.projection_matrix = make_projection_matrix(config);
        result.base.position = Vec3::new(0., 0., -3.5);

        result
    }

    pub fn update(&mut self) {
        if let Some(entity) = self.entity.clone() {
            self.base.position = Vec3::new(
                entity.position().x,
                entity.position().y + 0.6,
                entity.position().z
            );
            self.base.rotation = entity.rotation();
        }

        self.view_matrix = make_view_matrix(self);
        self.projection_view_matrix = self.projection_view_matrix * self.view_matrix;
        self.frustum.update(self.projection_view_matrix);
    }
    pub fn hook_entity(&mut self, entity: &dyn Entity) {
        self.entity = Some(uw_ref(entity));
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.view_matrix
    }
    pub fn proj_matrix(&self) -> Mat4 {
        self.projection_matrix
    }
    pub fn proj_view_matrix(&self) -> Mat4 {
        self.projection_view_matrix
    }

    pub fn frustum(&self) -> ViewFrustum {
        self.frustum
    }
}

impl Entity for Camera {
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