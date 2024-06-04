extern crate nalgebra_glm as glm;

use sfml::window::Key::P;
use crate::physics::aabb::AABB;

pub enum Planes {
    Near = 0,
    Far,
    Left,
    Right,
    Top,
    Bottom
}

/// @brief Vertex based construct, usually flat.
#[derive(Copy, Clone, Default)]
pub struct Plane {
    pub distance_to_origin: f32,
    pub normal: glm::TVec3<f32> // Vector3 normals
}

#[derive(Copy, Clone, Default)]
pub struct ViewFrustum {
    planes: [Plane; 6]
}

impl Plane {
    pub fn new(distance_to_origin: f32, normal: glm::TVec3<f32>) -> Self {
        Self { distance_to_origin, normal }
    }

    pub fn distance_to_point(&self, point: &glm::TVec3<f32>) -> f32 {
        glm::dot(point, &self.normal) + self.distance_to_origin
    }
}

impl ViewFrustum {
    pub fn new(planes: [Plane; 6]) -> Self {
        Self { planes }
    }

    /// @brief Updates the Frustrum relative between player and observed surface.
    /// @param mat
    pub fn update(&mut self, mat: &glm::TMat4<f32>) {
        // left
        self.planes[Planes::Left].normal.x = mat[(0, 3)] + mat[(0, 0)];
        self.planes[Planes::Left].normal.y = mat[(1, 3)] + mat[(1, 0)];
        self.planes[Planes::Left].normal.z = mat[(2, 3)] + mat[(2, 0)];
        self.planes[Planes::Left].distance_to_origin = mat[(3, 3)] + mat[(3, 0)];

        // right
        self.planes[Planes::Right].normal.x = mat[(0, 3)] - mat[(0, 0)];
        self.planes[Planes::Right].normal.y = mat[(1, 3)] - mat[(1, 0)];
        self.planes[Planes::Right].normal.z = mat[(2, 3)] - mat[(2, 0)];
        self.planes[Planes::Right].distance_to_origin = mat[(3, 3)] - mat[(3, 0)];

        // bottom
        self.planes[Planes::Bottom].normal.x = mat[(0, 3)] + mat[(0, 1)];
        self.planes[Planes::Bottom].normal.y = mat[(1, 3)] + mat[(1, 1)];
        self.planes[Planes::Bottom].normal.z = mat[(2, 3)] + mat[(2, 1)];
        self.planes[Planes::Bottom].distance_to_origin = mat[(3, 3)] + mat[(3, 1)];

        // top
        self.planes[Planes::Top].normal.x = mat[(0, 3)] - mat[(0, 1)];
        self.planes[Planes::Top].normal.y = mat[(1, 3)] - mat[(1, 1)];
        self.planes[Planes::Top].normal.z = mat[(2, 3)] - mat[(2, 1)];
        self.planes[Planes::Top].distance_to_origin = mat[(3, 3)] - mat[(3, 1)];

        // near
        self.planes[Planes::Near].normal.x = mat[(0, 3)] + mat[(0, 2)];
        self.planes[Planes::Near].normal.y = mat[(1, 3)] + mat[(1, 2)];
        self.planes[Planes::Near].normal.z = mat[(2, 3)] + mat[(2, 2)];
        self.planes[Planes::Near].distance_to_origin = mat[(3, 3)] + mat[(3, 2)];

        // far
        self.planes[Planes::Near].normal.x = mat[(0, 3)] + mat[(0, 3)];
        self.planes[Planes::Near].normal.y = mat[(1, 3)] + mat[(1, 3)];
        self.planes[Planes::Near].normal.z = mat[(2, 3)] + mat[(2, 3)];
        self.planes[Planes::Near].distance_to_origin = mat[(3, 3)] + mat[(3, 3)];

        for plane in self.planes.iter_mut() {
            let length = glm::length(&plane.normal);
            plane.normal /= length;
            plane.distance_to_origin /= length;
        }
    }

    /// @brief Determines if a collision box is present in the Frustrum.
    /// @param box
    /// @return result
    pub fn is_box_in_frustum(&self, box_: AABB) -> bool {
        let mut result = true;
        for plane in self.planes.iter() {
            if plane.distance_to_point(&box_.get_vp(plane.normal)) < 0.0 {
                return false;
            } else if plane.distance_to_point(&box_.get_vn(plane.normal)) < 0.0 {
                result = true;
            }
        }
        result
    }
}