use gl::types::GLuint;
use sfml::graphics::Image;

pub struct BasicTexture {
    id: GLuint
}

impl BasicTexture {
    fn load_from_image(&mut self, image: &Image) {
        //todo
    }
    fn load_from_file(&mut self, file: &str) {
        //todo
    }
    
    fn bind_texture(&self) {
        //todo
    }
}