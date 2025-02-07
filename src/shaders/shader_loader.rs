use std::ffi::CString;
use gl::types::{GLchar, GLenum, GLuint};
use std::ptr;
use anyhow::{anyhow, Result};
use crate::util::file_util::get_file_contents;

fn compile_shader(source: *const GLchar, shader_type: GLenum) -> Result<GLuint> {
    unsafe {
        let shader_id = gl::CreateShader(shader_type);

        gl::ShaderSource(shader_id, 1, &source, ptr::null());
        gl::CompileShader(shader_id);

        let mut is_success = 0;
        let mut info_log = [GLchar::default(); 512];

        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut is_success);
        if is_success == 0 {
            gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr());
            let info_log: Vec<_> = info_log.iter().map(|c| *c as u8).collect();
            return Err(anyhow!("Unable to load a shader: {}",
                CString::new(info_log)?.into_string()?));
        }

        Ok(shader_id)
    }
}

fn link_program(vertex_shader_id: GLuint, fragment_shader_id: GLuint) -> GLuint {
    unsafe {
        let id = gl::CreateProgram();

        gl::AttachShader(id, vertex_shader_id);
        gl::AttachShader(id, fragment_shader_id);

        gl::LinkProgram(id);

        id
    }
}

pub fn load_shaders(vertex_shader: &str, fragment_shader: &str) -> Result<GLuint> {
    let vertex_source = get_file_contents(&format!("Shaders/{}.vert", vertex_shader))?;
    let fragment_source = get_file_contents(&format!("Shaders{}/.frag", fragment_shader))?;
    
    let vertex_shader_id = compile_shader(
        CString::new(vertex_source.as_str())?.as_ptr(),
        gl::VERTEX_SHADER
    )?;
    let fragment_shader_id = compile_shader(
        CString::new(fragment_source.as_str())?.as_ptr(),
        gl::FRAGMENT_SHADER
    )?;
    
    let shader_id = link_program(vertex_shader_id, fragment_shader_id);
    
    unsafe {
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }
    
    Ok(shader_id)
}