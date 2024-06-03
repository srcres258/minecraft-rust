/// @brief Default configuration for program.
#[derive(Copy, Clone)]
pub struct Config {
    pub window_x: i32,
    pub window_y: i32,
    pub is_fullscreen: bool,
    pub render_distance: i32, // Set initial RD low to prevent long load times
    pub fov: i32
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_x: 1280,
            window_y: 720,
            is_fullscreen: false,
            render_distance: 8,
            fov: 90
        }
    }
}