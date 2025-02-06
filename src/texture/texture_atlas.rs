use std::ops::{Deref, DerefMut};
use gl::types::GLfloat;
use sfml::system::Vector2i;
use crate::texture::basic_texture::BasicTexture;

pub struct TextureAtlas {
    base: BasicTexture,

    image_size: i32,
    individual_texture_size: i32
}

impl Deref for TextureAtlas {
    type Target = BasicTexture;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for TextureAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TextureAtlas {
    pub fn new(texture_file_name: &str) -> Self {
        //todo
    }

    pub fn texture(&self, coords: Vector2i) -> [GLfloat; 8] {
        //todo
    }
}