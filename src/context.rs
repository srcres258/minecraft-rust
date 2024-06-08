use std::ffi::{c_void, CStr};
use std::ptr;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
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
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS); // makes sure errors are displayed synchronously
            gl::DebugMessageCallback(Some(gl_debug_output), ptr::null());
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, ptr::null(), gl::TRUE);
            
            gl::Viewport(0, 0, result.window.size().x as _, result.window.size().y as _);
            
            gl::CullFace(gl::BACK);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        result
    }
}

extern "system" fn gl_debug_output(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void
) {
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 { // ignore these non-significant error codes
        return;
    }

    let message_c_str;
    unsafe {
        message_c_str = CStr::from_ptr(message);
    }
    let message_str = message_c_str.to_str().unwrap();
    println!("Debug message ({}): {}", id, message_str);
    match source {
        gl::DEBUG_SOURCE_API => println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => {}
    }
    match gltype {
        gl::DEBUG_TYPE_ERROR => println!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => println!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => println!("Type: Other"),
        _ => {}
    }
    match severity {
        gl::DEBUG_SEVERITY_HIGH => println!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => println!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => println!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
        _ => {}
    }
    println!("---------------");
}