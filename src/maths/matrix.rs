use crate::camera::Camera;
use crate::config::Config;
use crate::entity::Entity;
use nalgebra_glm::{perspective, rotate, translate, Mat4, Vec3};

pub fn make_model_matrix(entity: &dyn Entity) -> Mat4 {
    let mut matrix = Mat4::default();
    
    matrix = rotate(&matrix, entity.rotation().x, &Vec3::new(1., 0., 0.));
    matrix = rotate(&matrix, entity.rotation().y, &Vec3::new(0., 1., 0.));
    matrix = rotate(&matrix, entity.rotation().z, &Vec3::new(0., 0., 1.));
    
    matrix = translate(&matrix, &entity.position());
    
    matrix
}

pub fn make_view_matrix(camera: &Camera) -> Mat4 {
    let mut matrix = Mat4::new_scaling(1.);

    matrix = rotate(&matrix, camera.rotation().x, &Vec3::new(1., 0., 0.));
    matrix = rotate(&matrix, camera.rotation().y, &Vec3::new(0., 1., 0.));
    matrix = rotate(&matrix, camera.rotation().z, &Vec3::new(0., 0., 1.));

    matrix = translate(&matrix, &(-camera.position()));
    
    matrix
}

pub fn make_projection_matrix(config: Config) -> Mat4 {
    let x = config.window_x as f32;
    let y = config.window_y as f32;
    let fov = config.fov as f32;
    
    perspective(fov.to_radians(), x / y, 0.1, 2000.0)
}