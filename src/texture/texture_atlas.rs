use std::ptr;
use gl::types::{GLfloat, GLuint};
use sfml::graphics::Image;
use sfml::system::Vector2i;

/// @brief Texture atlas that pulls texture data from existing files and maps them appropraitely.
#[derive(Default)]
pub struct TextureAtlas {
    id: GLuint,
    image_size: i32,
    individual_texture_size: i32
}

const IMAGE_SIZE: i32 = 256;
const INDIVIDUAL_TEXTURE_SIZE: i32 = 16;
const TEX_PER_ROW: GLfloat = IMAGE_SIZE as GLfloat / INDIVIDUAL_TEXTURE_SIZE as GLfloat;
const INDV_TEX_SIZE: GLfloat = 1f32 / TEX_PER_ROW;
const PIXEL_SIZE: GLfloat = 1f32 / IMAGE_SIZE as f32;

impl TextureAtlas {
    pub fn new(texture_file_name: &str) -> Self {
        let mut result = Self::default();

        let i = Image::from_file(format!("Res/Textures/{}.png", texture_file_name).as_str())
            .expect(format!("Unable to open image: {}", texture_file_name).as_str());
        result.load_from_image(&i);

        result.image_size = IMAGE_SIZE;
        result.individual_texture_size = INDIVIDUAL_TEXTURE_SIZE;

        result
    }

    pub fn load_from_image(&mut self, image: &Image) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            let pixel_data = image.pixel_data();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                image.size().x as _,
                image.size().y as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::addr_of!(pixel_data) as _
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 1);
        }
    }

    pub fn load_from_file(&mut self, file: &str) {
        let i = Image::from_file(format!("Res/Textures/{}.png", file).as_str())
            .expect(format!("Unable to load BasicTexture: {}", file).as_str());
        self.load_from_image(&i);
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn get_texture(&self, coords: &Vector2i) -> [GLfloat; 8] {
        let x_min = coords.x as f32 * INDV_TEX_SIZE + 0.5 * PIXEL_SIZE;
        let y_min = coords.y as f32 * INDV_TEX_SIZE + 0.5 * PIXEL_SIZE;

        let x_max = x_min + INDV_TEX_SIZE - PIXEL_SIZE;
        let y_max = y_min + INDV_TEX_SIZE - PIXEL_SIZE;

        [x_max, y_max, x_min, y_max, x_min, y_min, x_max, y_min]
    }
}

impl Drop for TextureAtlas {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}