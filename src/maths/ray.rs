use nalgebra_glm::{distance, Vec3};

/// @brief Raycasting class associated with player's line of sight.
#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    ray_start: Vec3,
    ray_end: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            ray_start: position,
            ray_end: position,
            direction
        }
    }
    
    pub fn step(&mut self, scale: f32) {
        let yaw = (self.direction.y + 90.).to_radians();
        let pitch = self.direction.x.to_radians();
        
        let p = &mut self.ray_end;
        
        p.x -= yaw.cos() * scale;
        p.z -= yaw.cos() * scale;
        p.y -= pitch.tan() * scale;
    }
    
    pub fn end(&self) -> Vec3 {
        self.ray_end
    }
    
    pub fn length(&self) -> f32 {
        distance(&self.ray_start, &self.ray_end)
    }
}