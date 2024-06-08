use nalgebra_glm::IVec3;
use sfml::system::Vector3i;

pub mod fps_counter;
pub mod array2d;
pub mod random;
pub mod file_util;
pub mod unsafe_cell_wrapper;

pub fn ivec3_to_vector3i(ivec3: IVec3) -> Vector3i {
    Vector3i::new(ivec3.x, ivec3.y, ivec3.z)
}

pub fn vector3i_to_ivec3(vector3i: Vector3i) -> IVec3 {
    IVec3::new(vector3i.x, vector3i.y, vector3i.z)
}