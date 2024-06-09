use std::ffi::CString;
use std::{mem, ptr};
use gl::types::{GLsizei, GLsizeiptr};
use sfml::window::Window;
use crate::camera::Camera;
use crate::renderer::chunk_renderer::ChunkRenderer;
use crate::renderer::flora_renderer::FloraRenderer;
use crate::renderer::skybox_renderer::SkyboxRenderer;
use crate::renderer::water_renderer::WaterRenderer;
use crate::world::chunk::chunk_section::ChunkSection;

/// @brief Master rendering class that handles the sum of drawn in-game objects.
#[derive(Default)]
pub struct RenderMaster {
    // Chunks
    chunk_renderer: ChunkRenderer,
    water_renderer: WaterRenderer,
    flora_renderer: FloraRenderer,

    // Detail
    skybox_renderer: SkyboxRenderer,
    
    draw_box: bool
}

const VERTEX_SHADER_SOURCE: &str = r##"#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}"##;

const FRAGMENT_SHADER_SOURCE: &str = r##"#version 330 core
out vec4 FragColor;
void main()
{
   FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}"##;

impl RenderMaster {
    pub fn draw_chunk(&mut self, chunk: &ChunkSection) {
        let solid_mesh = &chunk.get_meshes().solid_mesh;
        let water_mesh = &chunk.get_meshes().water_mesh;
        let flora_mesh = &chunk.get_meshes().flora_mesh;

        if solid_mesh.faces > 0 {
            self.chunk_renderer.add(solid_mesh);
        }

        if water_mesh.faces > 0 {
            self.water_renderer.add(water_mesh);
        }

        if flora_mesh.faces > 0 {
            self.flora_renderer.add(flora_mesh);
        }
    }

    pub fn draw_sky(&mut self) {
        self.draw_box = true;
    }

    pub fn finish_render(&mut self, window: &mut Window, camera: &Camera) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
        }

        render_test();
        
        self.chunk_renderer.render(camera);
        self.water_renderer.render(camera);
        self.flora_renderer.render(camera);

        if self.draw_box {
            unsafe {
                gl::Disable(gl::CULL_FACE);
            }
            self.skybox_renderer.render(camera);
            self.draw_box = false;
        }

        window.display();
    }
}

fn render_test() {
    unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_data_source = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_data_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        // check for shader compile errors
        let mut success = 0i32;
        let mut info_log = [0i8; 512];
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), &mut info_log as *mut _);
            let info_log_vec: Vec<_> = Vec::from(info_log).iter().map(|it| *it as u8).collect();
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8(info_log_vec).unwrap());
        }
        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_data_source = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fragment_data_source.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        // check for shader compile errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), &mut info_log as *mut _);
            let info_log_vec: Vec<_> = Vec::from(info_log).iter().map(|it| *it as u8).collect();
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8(info_log_vec).unwrap());
        }
        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(shader_program, 512, ptr::null_mut(), &mut info_log as *mut _);
            let info_log_vec: Vec<_> = Vec::from(info_log).iter().map(|it| *it as u8).collect();
            println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", String::from_utf8(info_log_vec).unwrap());
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices = [
            -0.5f32, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0  // top
        ];

        let (mut vbo, mut vao) = (0u32, 0u32);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
            ptr::addr_of!(vertices) as *const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * mem::size_of::<f32>()) as GLsizei,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // draw our first triangle
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao); // // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // gl::BindVertexArray(0); // no need to unbind it every time

        // optional: de-allocate all resources once they've outlived their purpose:
        // ------------------------------------------------------------------------
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteProgram(shader_program);
    }
}