use std::mem;
use nalgebra_glm::{dot, length, Mat4, Vec3};
use crate::physics::aabb::AABB;

#[derive(Copy, Clone, Debug)]
pub enum Planes {
    Near = 0,
    Far,
    Left,
    Right,
    Top,
    Bottom
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Plane {
    pub distance_to_origin: f32,
    pub normal: Vec3
}

#[derive(Copy, Clone, Default, Debug)]
pub struct ViewFrustum {
    planes: [Plane; mem::variant_count::<Planes>()]
}

/// @brief Vertex based construct, usually flat.
impl Plane {
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        dot(&point, &self.normal) + self.distance_to_origin
    }
}

impl ViewFrustum {
    /// @brief Updates the Frustrum relative between player and observed surface.
    /// @param mat
    pub fn update(&mut self, mat: Mat4) {
        // Note that the index order in nalgebra_glm lib in Rust is **reversed**
        // compared to glm lib in C++.
        // For more details see `skybox_shader.rs` file.

        // left
        self.planes[Planes::Left as usize].normal.x = mat[(3, 0)] + mat[(0, 0)];
        self.planes[Planes::Left as usize].normal.y = mat[(3, 1)] + mat[(0, 1)];
        self.planes[Planes::Left as usize].normal.z = mat[(3, 2)] + mat[(0, 2)];
        self.planes[Planes::Left as usize].distance_to_origin = mat[(3, 3)] + mat[(0, 2)];

        // right
        self.planes[Planes::Right as usize].normal.x = mat[(3, 0)] - mat[(0, 0)];
        self.planes[Planes::Right as usize].normal.y = mat[(3, 1)] - mat[(0, 1)];
        self.planes[Planes::Right as usize].normal.z = mat[(3, 2)] - mat[(0, 2)];
        self.planes[Planes::Right as usize].distance_to_origin = mat[(3, 3)] - mat[(0, 3)];

        // bottom
        self.planes[Planes::Bottom as usize].normal.x = mat[(3, 0)] + mat[(1, 0)];
        self.planes[Planes::Bottom as usize].normal.y = mat[(3, 1)] + mat[(1, 1)];
        self.planes[Planes::Bottom as usize].normal.z = mat[(3, 2)] + mat[(1, 2)];
        self.planes[Planes::Bottom as usize].distance_to_origin = mat[(3, 3)] + mat[(1, 3)];

        // top
        self.planes[Planes::Top as usize].normal.x = mat[(3, 0)] - mat[(1, 0)];
        self.planes[Planes::Top as usize].normal.y = mat[(3, 1)] - mat[(1, 1)];
        self.planes[Planes::Top as usize].normal.z = mat[(3, 2)] - mat[(1, 2)];
        self.planes[Planes::Top as usize].distance_to_origin = mat[(3, 3)] - mat[(1, 3)];

        // near
        self.planes[Planes::Near as usize].normal.x = mat[(3, 0)] + mat[(2, 0)];
        self.planes[Planes::Near as usize].normal.y = mat[(3, 1)] + mat[(2, 1)];
        self.planes[Planes::Near as usize].normal.z = mat[(3, 2)] + mat[(2, 2)];
        self.planes[Planes::Near as usize].distance_to_origin = mat[(3, 3)] + mat[(2, 3)];

        // far
        self.planes[Planes::Far as usize].normal.x = mat[(3, 0)] - mat[(2, 0)];
        self.planes[Planes::Far as usize].normal.y = mat[(3, 1)] - mat[(2, 1)];
        self.planes[Planes::Far as usize].normal.z = mat[(3, 2)] - mat[(2, 2)];
        self.planes[Planes::Far as usize].distance_to_origin = mat[(3, 3)] - mat[(2, 3)];

        for plane in self.planes.iter_mut() {
            let length = length(&plane.normal);
            plane.normal /= length;
            plane.distance_to_origin /= length;
        }
    }

    /// @brief Determines if a collision box is present in the Frustrum.
    /// @param box
    /// @return result
    pub fn is_box_in_frustum(&self, box_aabb: AABB) -> bool {
        for plane in self.planes.iter() {
            if plane.distance_to_point(box_aabb.vp(plane.normal)) < 0. {
                return false;
            } else if plane.distance_to_point(box_aabb.vn(plane.normal)) < 0. {
                return true;
            }
        }
        true
    }
}