// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
#[derive(Copy, Clone, Default, Debug)]
pub struct Plane {
    pub distance_to_origin: f32,
    pub normal: glm::TVec3<f32> // Vector3 normals
}

#[derive(Copy, Clone, Default, Debug)]
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
        self.planes[Planes::Left as usize].distance_to_origin = mat[(3, 3)] + mat[(0, 3)];

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
            let length = glm::length(&plane.normal);
            plane.normal /= length;
            plane.distance_to_origin /= length;
        }
    }

    /// @brief Determines if a collision box is present in the Frustrum.
    /// @param box
    /// @return result
    pub fn is_box_in_frustum(&self, box_: AABB) -> bool {
        for plane in self.planes.iter() {
            if plane.distance_to_point(&box_.get_vp(plane.normal)) < 0.0 {
                return false;
            } else if plane.distance_to_point(&box_.get_vn(plane.normal)) < 0.0 {
                continue;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn test_config() -> Config {
        Config {
            render_distance: 8,
            is_fullscreen: false,
            window_x: 1600,
            window_y: 900,
            fov: 105,
        }
    }

    fn identity_matrix() -> glm::TMat4<f32> {
        glm::diagonal4x4(&glm::vec4(1.0_f32, 1.0, 1.0, 1.0))
    }

    #[test]
    fn test_frustum_plane_extraction_identity() {
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&identity_matrix());

        let expected = [
            (Planes::Near as usize, glm::vec3(0.0, 0.0, 1.0), 1.0),
            (Planes::Far as usize, glm::vec3(0.0, 0.0, -1.0), 1.0),
            (Planes::Left as usize, glm::vec3(1.0, 0.0, 0.0), 1.0),
            (Planes::Right as usize, glm::vec3(-1.0, 0.0, 0.0), 1.0),
            (Planes::Top as usize, glm::vec3(0.0, -1.0, 0.0), 1.0),
            (Planes::Bottom as usize, glm::vec3(0.0, 1.0, 0.0), 1.0),
        ];

        for (index, normal, distance) in expected {
            let plane = frustum.planes[index];
            assert!((glm::length(&plane.normal) - 1.0).abs() < 0.0001);
            assert!((plane.normal.x - normal.x).abs() < 0.0001);
            assert!((plane.normal.y - normal.y).abs() < 0.0001);
            assert!((plane.normal.z - normal.z).abs() < 0.0001);
            assert!((plane.distance_to_origin - distance).abs() < 0.0001);
        }
    }

    #[test]
    fn test_frustum_plane_extraction_projection_matrix() {
        let config = test_config();
        let proj = crate::maths::matrix::make_projection_matrix(&config);
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&proj);

        for plane in frustum.planes.iter() {
            let len = glm::length(&plane.normal);
            assert!((len - 1.0).abs() < 0.0001, "plane not normalized: {len}");
            assert!(!plane.distance_to_origin.is_nan());
        }
    }

    #[test]
    fn test_aabb_inside_frustum() {
        let config = test_config();
        let proj = crate::maths::matrix::make_projection_matrix(&config);
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&proj);

        let mut box_ = AABB::new(&glm::vec3(0.5, 0.5, 0.5));
        box_.update(&glm::vec3(0.0, 0.0, -2.0));
        assert!(frustum.is_box_in_frustum(box_));
    }

    #[test]
    fn test_aabb_behind_near_plane() {
        let config = test_config();
        let proj = crate::maths::matrix::make_projection_matrix(&config);
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&proj);

        let mut box_ = AABB::new(&glm::vec3(0.5, 0.5, 0.5));
        box_.update(&glm::vec3(0.0, 0.0, 5.0));
        assert!(!frustum.is_box_in_frustum(box_));
    }

    #[test]
    fn test_aabb_outside_side_plane() {
        let config = test_config();
        let proj = crate::maths::matrix::make_projection_matrix(&config);
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&proj);

        let mut box_ = AABB::new(&glm::vec3(1.0, 1.0, 1.0));
        box_.update(&glm::vec3(2000.0, 0.0, -500.0));
        assert!(!frustum.is_box_in_frustum(box_));
    }

    #[test]
    fn test_all_planes_checked_for_accept_and_reject_cases() {
        let config = test_config();
        let proj = crate::maths::matrix::make_projection_matrix(&config);
        let mut frustum = ViewFrustum::new([Plane::default(); 6]);
        frustum.update(&proj);

        let mut inside = AABB::new(&glm::vec3(1.0, 1.0, 1.0));
        inside.update(&glm::vec3(0.0, 0.0, -10.0));
        assert!(frustum.is_box_in_frustum(inside));

        let mut outside = AABB::new(&glm::vec3(1.0, 1.0, 1.0));
        outside.update(&glm::vec3(-2000.0, 0.0, -10.0));
        assert!(!frustum.is_box_in_frustum(outside));
    }
}
