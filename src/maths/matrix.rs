extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::config::Config;
use crate::entity::Entity;

pub fn make_model_matrix(entity: &Entity) -> glm::TMat4<f32> {
    let mut matrix: glm::TMat4<f32> = Default::default();

    matrix = glm::rotate(&matrix, entity.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
    matrix = glm::rotate(&matrix, entity.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
    matrix = glm::rotate(&matrix, entity.rotation.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0));

    matrix = glm::translate(&matrix, &entity.position);

    matrix
}

pub fn make_view_matrix(camera: &Camera) -> glm::TMat4<f32> {
    let mut matrix = glm::diagonal4x4(&glm::vec4(1.0f32, 1.0, 1.0, 1.0));

    matrix = glm::rotate(&matrix, camera.wrapped_obj.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
    matrix = glm::rotate(&matrix, camera.wrapped_obj.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
    matrix = glm::rotate(&matrix, camera.wrapped_obj.rotation.z.to_radians(), &glm::vec3(1.0, 0.0, 1.0));

    matrix = glm::translate(&matrix, &(-camera.wrapped_obj.position));

    matrix
}

pub fn make_projection_matrix(config: &Config) -> glm::TMat4<f32> {
    let x = config.window_x as f32;
    let y = config.window_y as f32;
    let fov = config.fov as f32;
    
    glm::perspective(fov.to_radians(), x / y, 0.1, 2000.0)
}