extern crate nalgebra_glm as glm;

/// @brief Raycasting class associated with player's line of sight.
pub struct Ray {
    ray_start: glm::TVec3<f32>,
    ray_end: glm::TVec3<f32>,
    direction: glm::TVec3<f32>
}

impl Ray {
    pub fn new(position: &glm::TVec3<f32>, direction: &glm::TVec3<f32>) -> Self {
        Self {
            ray_start: *position,
            ray_end: *position,
            direction: *direction
        }
    }

    pub fn step(&mut self, scale: f32) {
        let yaw = (self.direction.y + 90.0).to_radians();
        let pitch = self.direction.x.to_radians();

        self.ray_end.x -= yaw.cos() * scale;
        self.ray_end.z -= yaw.sin() * scale;
        self.ray_end.y -= pitch.tan() * scale;
    }

    pub fn end(&self) -> glm::TVec3<f32> {
        self.ray_end
    }

    pub fn length(&self) -> f32 {
        glm::distance(&self.ray_start, &self.ray_end)
    }
}