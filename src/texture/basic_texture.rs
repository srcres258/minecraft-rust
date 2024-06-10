// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gl::types::GLuint;
use sfml::graphics::Image;

/// @brief Standard texture that will be mapped to objects.
#[derive(Default)]
pub struct BasicTexture {
    id: GLuint
}

impl BasicTexture {
    pub fn new(file: &str) -> Self {
        let mut result = Self::default();
        result.load_from_file(file);
        result
    }

    pub fn load_from_image(&mut self, i: &Image) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            let pixel_data = i.pixel_data();
            let pixel_data_vec = Vec::from(pixel_data);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                i.size().x as _,
                i.size().y as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixel_data_vec.as_ptr() as _
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
}

impl Drop for BasicTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}