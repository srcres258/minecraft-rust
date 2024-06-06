use std::ffi::CString;
use std::ptr;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use crate::util::file_util::get_file_contents;

fn compile_shader(source: *const GLchar, shader_type: GLenum) -> Result<GLuint, String> {
    let shader_id;

    unsafe {
        shader_id = gl::CreateShader(shader_type);

        gl::ShaderSource(shader_id, 1, &source, ptr::null());
        gl::CompileShader(shader_id);

        let mut is_success: GLint = 0;
        let mut info_log: [GLchar; 512] = [0; 512];

        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut is_success);
        if is_success == 0 {
            gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), ptr::addr_of_mut!(info_log) as _);
            let info_log: Vec<_> = info_log.iter().map(|c| *c as u8).collect();
            return Err(format!("Unable to load a shader: {}", CString::new(info_log).unwrap().into_string().unwrap()));
        }
    }

    Ok(shader_id)
}

fn link_program(vertex_shader_id: GLuint, fragment_shader_id: GLuint) -> GLuint {
    let id;
    
    unsafe {
        id = gl::CreateProgram();
        
        gl::AttachShader(id, vertex_shader_id);
        gl::AttachShader(id, fragment_shader_id);
        
        gl::LinkProgram(id);
    }
    
    id
}

pub fn load_shaders(vertex_shader: &str, fragment_shader: &str) -> GLuint {
    let vertex_source = get_file_contents(format!("Shaders/{}.vert", vertex_shader).as_str());
    let fragment_source = get_file_contents(format!("Shaders/{}.frag", fragment_shader).as_str());
    
    let vertex_source = CString::new(vertex_source).unwrap();
    let fragment_source = CString::new(fragment_source).unwrap();
    
    let vertex_shader_id = compile_shader(vertex_source.as_ptr(), gl::VERTEX_SHADER).unwrap();
    let fragment_shader_id = compile_shader(fragment_source.as_ptr(), gl::FRAGMENT_SHADER).unwrap();
    
    let shader_id = link_program(vertex_shader_id, fragment_shader_id);
    
    unsafe {
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }
    
    shader_id
}