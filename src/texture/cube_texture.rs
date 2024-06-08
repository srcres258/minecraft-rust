use gl::types::{GLenum, GLuint};
use sfml::graphics::Image;

/// @brief Complex texture class that applies textures to all sides of a cube.
#[derive(Default)]
pub struct CubeTexture {
    tex_id: GLuint
}

impl CubeTexture {
    pub fn new(files: [&str; 6]) -> Self {
        let mut result = Self::default();
        result.load_from_files(files);
        result
    }

    pub fn load_from_files(&mut self, files: [&str; 6]) {
        unsafe {
            gl::GenTextures(1, &mut self.tex_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.tex_id);

            for (i, s) in files.iter().enumerate() {
                let image = Image::from_file(format!("Res/Textures/{}.png", s).as_str())
                    .expect(format!("Unable to load CubeTexture Part: {}", s).as_str());

                let param = gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as GLenum;
                let width = image.size().x;
                let height = image.size().y;

                let pixel_data = image.pixel_data();
                let pixel_data_vec = Vec::from(pixel_data);
                gl::TexImage2D(
                    param,
                    0,
                    gl::RGBA as _,
                    width as _,
                    height as _,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    pixel_data_vec.as_ptr() as _
                );
            }

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
        }
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.tex_id);
        }
    }
}

impl Drop for CubeTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.tex_id);
        }
    }
}