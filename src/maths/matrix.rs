extern crate nalgebra_glm as glm;

use nalgebra_glm::TMat4;
use crate::camera::Camera;
use crate::config::Config;
use crate::entity::Entity;

fn print_matrix(mat: TMat4<f32>) {
    println!("{} {} {} {}", mat[(0, 0)], mat[(1, 0)], mat[(2, 0)], mat[(3, 0)]);
    println!("{} {} {} {}", mat[(0, 1)], mat[(1, 1)], mat[(2, 1)], mat[(3, 1)]);
    println!("{} {} {} {}", mat[(0, 2)], mat[(1, 2)], mat[(2, 2)], mat[(3, 2)]);
    println!("{} {} {} {}", mat[(0, 3)], mat[(1, 3)], mat[(2, 3)], mat[(3, 3)]);
}

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

    println!("Camera rotation: {} {} {}", camera.rotation.x, camera.rotation.y, camera.rotation.z);
    println!("Camera position: {} {} {}", camera.position.x, camera.position.y, camera.position.z);

    matrix = glm::rotate(&matrix, camera.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
    matrix = glm::rotate(&matrix, camera.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
    matrix = glm::rotate(&matrix, camera.rotation.z.to_radians(), &glm::vec3(1.0, 0.0, 1.0));

    matrix = glm::translate(&matrix, &(-camera.position));

    println!("------- view matrix -------");
    print_matrix(matrix);
    println!("---------------------------");

    matrix
}

pub fn make_projection_matrix(config: &Config) -> glm::TMat4<f32> {
    let x = config.window_x as f32;
    let y = config.window_y as f32;
    let fov = config.fov as f32;

    println!("------- proj matrix -------");
    print_matrix(glm::perspective(fov.to_radians(), x / y, 0.1, 2000.0));
    println!("---------------------------");
    
    glm::perspective(fov.to_radians(), x / y, 0.1, 2000.0)
}