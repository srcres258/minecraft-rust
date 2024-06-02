/// @brief Default configuration for program.
pub struct Config {
    window_x: i32,
    window_y: i32,
    is_fullscreen: bool,
    render_distance: i32, // Set initial RD low to prevent long load times
    fov: i32
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