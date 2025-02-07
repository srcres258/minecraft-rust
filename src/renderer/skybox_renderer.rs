use gl::types::{GLfloat, GLuint};
use crate::model::Model;
use crate::shaders::skybox_shader::SkyboxShader;
use crate::texture::cube_texture::CubeTexture;
use anyhow::Result;
use crate::camera::Camera;
use crate::gl::gl_functions::draw_elements;
use crate::shaders::shader::Shader;

/// @brief Renderer that specifically draws the skybox and entities outside player reach.
pub struct SkyboxRenderer {
    sky_cube: Model,
    shader: SkyboxShader,
    cube_texture: CubeTexture
}

const SIZE: GLfloat = 500.;

impl SkyboxRenderer {
    pub fn new() -> Result<Self> {
        let mut result = Self {
            sky_cube: Default::default(),
            shader: Default::default(),
            cube_texture: Default::default()
        };

        let vertex_coords = Vec::from([
            // Back
            SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,

            // Front
            -SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,

            // Right
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            SIZE,

            // Left
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            SIZE,
            -SIZE,
            SIZE,
            -SIZE,

            // Top
            -SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,

            // Bottom
            -SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            SIZE,
            -SIZE,
            -SIZE,
            SIZE
        ]);

        let indices: Vec<GLuint> = [
            0,  1,  2,  2,  3,  0,

            4,  5,  6,  6,  7,  4,

            8,  9,  10, 10, 11, 8,

            12, 13, 14, 14, 15, 12,

            16, 17, 18, 18, 19, 16,

            20, 21, 22, 22, 23, 20
        ].into();

        result.sky_cube.gen_vao();
        result.sky_cube.add_vbo(3, &vertex_coords);
        result.sky_cube.add_ebo(&indices);

        result.cube_texture.load_from_files([
            "dm",
            "dm",
            "dt",
            "db",
            "dm",
            "dm"
        ])?;

        Ok(result)
    }

    pub fn render(&self, camera: &Camera) {
        self.shader.use_program();
        self.sky_cube.bind_vao();
        self.cube_texture.bind_texture();

        self.shader.load_view_matrix(camera.view_matrix());
        self.shader.load_projection_matrix(camera.proj_matrix());

        draw_elements(self.sky_cube.indices_count());
    }
}