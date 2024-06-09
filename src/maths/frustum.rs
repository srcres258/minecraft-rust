extern crate nalgebra_glm as glm;

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
        /*
        NOTE that within the nalgebra_glm crate, implementation of matrix indexing is reversed
        compared to that of C++ glm library.

        e.g. In C++ glm library we use `mat[x][y]` to refer to the element at column x and row y
        of the matrix. However, it is represented as `mat[(y, x)]` in nalgebra_glm.

        Hence, be CAREFUL about the difference between the two libraries.
         */

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
        self.planes[Planes::Near as usize].normal.x = mat[(3, 0)] - mat[(2, 0)];
        self.planes[Planes::Near as usize].normal.y = mat[(3, 1)] - mat[(2, 1)];
        self.planes[Planes::Near as usize].normal.z = mat[(3, 2)] - mat[(2, 2)];
        self.planes[Planes::Near as usize].distance_to_origin = mat[(3, 3)] - mat[(2, 3)];

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