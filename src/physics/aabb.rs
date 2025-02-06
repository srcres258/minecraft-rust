use nalgebra_glm::Vec3;

/// @brief Collision detection class for 3D environment.
#[derive(Copy, Clone, Default)]
pub struct AABB {
    pub position: Vec3,
    pub dimensions: Vec3
}

impl AABB {
    pub fn new(dim: Vec3) -> Self {
        Self {
            position: Vec3::default(),
            dimensions: dim
        }
    }

    pub fn update(&mut self, location: Vec3) {
        self.position = location;
    }

    pub fn vn(&self, normal: Vec3) -> Vec3 {
        let mut res = self.position;

        if normal.x < 0.0 {
            res.x += self.dimensions.x;
        }
        if normal.y < 0.0 {
            res.y += self.dimensions.y;
        }
        if normal.z < 0.0 {
            res.z += self.dimensions.z;
        }

        res
    }

    pub fn vp(&self, normal: Vec3) -> Vec3 {
        let mut res = self.position;

        if normal.x > 0.0 {
            res.x += self.dimensions.x;
        }
        if normal.y > 0.0 {
            res.y += self.dimensions.y;
        }
        if normal.z > 0.0 {
            res.z += self.dimensions.z;
        }

        res
    }
}