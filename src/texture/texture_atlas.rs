use crate::texture::basic_texture::{BasicTexture, BasicTextureImpl};
use delegate::delegate;
use gl::types::GLfloat;
use sfml::graphics::Image;
use sfml::system::Vector2i;
use anyhow::{anyhow, Result};

pub struct TextureAtlas {
    base: BasicTextureImpl,

    image_size: i32,
    individual_texture_size: i32
}

impl TextureAtlas {
    pub fn new(texture_file_name: &str) -> Result<Self> {
        let mut result = Self {
            base: BasicTextureImpl::default(),
            image_size: 256,
            individual_texture_size: 16
        };

        let image = Image::from_file(&format!("Res/Textures/{}.png", texture_file_name))
            .ok_or(anyhow!("Unable to open image: {}", texture_file_name))?;
        result.load_from_image(&image);

        Ok(result)
    }

    pub fn texture(&self, coords: Vector2i) -> [GLfloat; 8] {
        let tex_per_row = self.image_size as GLfloat / self.individual_texture_size as GLfloat;
        let indv_tex_size = 1.0 / tex_per_row;
        let pixel_size = 1.0 / self.image_size as GLfloat;

        let x_min = coords.x as GLfloat * indv_tex_size + 0.5 * pixel_size;
        let y_min = coords.y as GLfloat * indv_tex_size + 0.5 * pixel_size;

        let x_max = x_min + indv_tex_size - pixel_size;
        let y_max = y_min + indv_tex_size - pixel_size;

        [x_max, y_max, x_min, y_max, x_min, y_min, x_max, y_min]
    }
}

impl BasicTexture for TextureAtlas {
    delegate! {
        to self.base {
            fn load_from_image(&mut self, image: &Image);
            fn load_from_file(&mut self, file: &str);

            fn bind_texture(&self);
        }
    }
}