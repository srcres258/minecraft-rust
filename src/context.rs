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
                Window::new(VideoMode::desktop_mode(), "minecraft-rust", Style::FULLSCREEN, &settings)
            } else {
                let win_mode = VideoMode::new(config.window_x as _, config.window_y as _, 32);
                Window::new(win_mode, "minecraft-rust", Style::CLOSE, &settings)
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
    log::debug!("Debug message ({}): {}", id, message_str);
    match source {
        gl::DEBUG_SOURCE_API => log::debug!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => log::debug!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => log::debug!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => log::debug!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => log::debug!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => log::debug!("Source: Other"),
        _ => {}
    }
    match gltype {
        gl::DEBUG_TYPE_ERROR => log::debug!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => log::debug!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => log::debug!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => log::debug!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => log::debug!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => log::debug!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => log::debug!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => log::debug!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => log::debug!("Type: Other"),
        _ => {}
    }
    match severity {
        gl::DEBUG_SEVERITY_HIGH => log::debug!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => log::debug!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => log::debug!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => log::debug!("Severity: notification"),
        _ => {}
    }
    log::debug!("---------------");
}