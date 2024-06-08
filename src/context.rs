use sfml::window::{ContextSettings, Style, VideoMode, Window};
use crate::config::Config;

pub struct Context {
    pub window: Window
}

impl Context {
    pub fn new(config: Config) -> Self {
        let mut settings = ContextSettings::default();
        settings.antialiasing_level = 0;
        settings.major_version = 3;
        settings.minor_version = 3;
        settings.depth_bits = 24;
        settings.stencil_bits = 8;

        let result = Self {
            window: if config.is_fullscreen {
                Window::new(VideoMode::desktop_mode(), "MineCraft Week", Style::CLOSE, &settings)
            } else {
                let win_mode = VideoMode::new(config.window_x as _, config.window_y as _, 32);
                Window::new(win_mode, "MineCraft Week", Style::CLOSE, &settings)
            }
        };

        // Load OpenGL library.
        gl_loader::init_gl();
        // Load all the OpenGL function pointer using the `gl` crate.
        gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
        // Unload the OpenGL library.
        gl_loader::end_gl();
        
        unsafe {
            gl::Viewport(0, 0, result.window.size().x as _, result.window.size().y as _);
            
            gl::CullFace(gl::BACK);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        result
    }
}