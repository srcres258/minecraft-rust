use gl::types::GLuint;
use sfml::graphics::Image;
use anyhow::{anyhow, Result};

pub trait BasicTexture {
    fn load_from_image(&mut self, image: &Image);
    fn load_from_file(&mut self, file: &str) -> Result<()>;

    fn bind_texture(&self);
}

#[derive(Default)]
pub struct BasicTextureImpl {
    id: GLuint
}

impl BasicTextureImpl {
    pub fn new(file: &str) -> Result<Self> {
        let mut result = Self::default();
        result.load_from_file(file)?;
        Ok(result)
    }
}

impl BasicTexture for BasicTextureImpl {
    fn load_from_image(&mut self, image: &Image) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                image.size().x as _,
                image.size().y as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.pixel_data().as_ptr() as _
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                              gl::NEAREST_MIPMAP_NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                              gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,
                              gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T,
                              gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 1);
        }
    }
    fn load_from_file(&mut self, file: &str) -> Result<()> {
        let image = Image::from_file(&format!("Res/Textures/{}.png", file))
            .ok_or(anyhow!("Unable to load BasicTexture: {}", file))?;

        self.load_from_image(&image);

        Ok(())
    }

    fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for BasicTextureImpl {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}