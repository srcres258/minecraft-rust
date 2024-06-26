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

use sfml::graphics::{Color, Font, Text, Transformable};
use sfml::SfBox;
use sfml::system::{Clock, Vector2f};
use crate::renderer::render_master::RenderMaster;

/// @brief Generally obsolete FPS counter associated with SFML.
pub struct FPSCounter<'a> {
    enabled: bool,
    debugging: bool,

    text: Text<'a>,
    font: SfBox<Font>,

    delay_timer: SfBox<Clock>,
    fps_timer: SfBox<Clock>,

    fps: f32,

    frame_count: i32
}

impl<'a> FPSCounter<'a> {
    pub fn new() -> Self {
        let mut result = Self {
            enabled: true,
            debugging: false,
            text: Text::default(),
            font: Font::from_file("Res/Fonts/rs.ttf").unwrap(),
            delay_timer: Clock::start(),
            fps_timer: Clock::start(),
            fps: 0.0,
            frame_count: 0
        };

        result.text.move_(Vector2f::new(10.0, 10.0));
        result.text.set_outline_color(Color::BLACK);
        result.text.set_outline_thickness(2.0);

        result.text.set_character_size(25);

        result
    }

    pub fn init(&'a mut self) {
        self.text.set_font(&self.font);
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        if self.enabled {
            if self.delay_timer.elapsed_time().as_seconds() > 0.5 {
                self.fps = self.frame_count as f32 / self.fps_timer.restart().as_seconds();
                self.frame_count = 0;
                self.delay_timer.restart();

                // Only show this output in debug mode
                if self.debugging {
                    log::debug!("{}", self.fps);
                }
            }
        }
    }

    pub fn draw(&mut self, _renderer: &RenderMaster) {
        self.text.set_string(format!("FPS: {}", self.fps).as_str());
    }
}