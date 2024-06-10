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

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use crate::application::Application;
use crate::config::Config;

pub mod application;
pub mod states;
pub mod context;
pub mod renderer;
pub mod camera;
pub mod entity;
pub mod physics;
pub mod config;
pub mod input;
pub mod player;
pub mod item;
pub mod world;
pub mod util;
pub mod maths;
pub mod mesh;
pub mod model;
pub mod texture;
pub mod shaders;
pub mod gl;

/// @brief Self declared function that loads in configuration files as needed.
/// @param config
fn load_config(config: &mut Config) {
    let config_file = Path::new("config.txt");

    // If the config file is missing or "bad"
    if !config_file.exists() {
        println!("Configuration file invalid,");
        println!("writing 'new' configuration.");
        println!();

        let out_file = File::create(config_file).unwrap();
        let mut writer = BufWriter::new(out_file);
        writer.write(b"renderdistance 8\n").unwrap();
        writer.write(b"fullscreen 0\n").unwrap();
        writer.write(b"windowsize 1600 900\n").unwrap();
        writer.write(b"fov 105\n").unwrap();
        writer.flush().unwrap();

        println!();
        println!("New configuration file created.");
    }

    // Open 'new' config file.
    let config_file = File::open(config_file)
        .expect("Unable to load configuration file.");
    let reader = BufReader::new(config_file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let parts: Vec<_> = line.split(' ').collect();
        if parts.len() > 0 {
            let key = parts[0];
            if key == "renderdistance" {
                config.render_distance = parts[1].parse().unwrap();
                println!("Config: Render Distance: {}", config.render_distance);
            } else if key == "fullscreen" {
                let is_fullscreen: i32 = parts[1].parse().unwrap();
                config.is_fullscreen = is_fullscreen == 1;
                println!("Config: Full screen mode: {}", config.is_fullscreen);
            } else if key == "windowsize" {
                config.window_x = parts[1].parse().unwrap();
                config.window_y = parts[2].parse().unwrap();
                println!("Config: Window Size: {} x {}", config.window_x, config.window_y);
            } else if key == "fov" {
                config.fov = parts[1].parse().unwrap();
                println!("Config: Field of Vision: {}", config.fov);
            }
        }
    }
}

fn display_info() {
    let info = fs::read_to_string("Res/info.txt").unwrap();
    println!("{}", info);
}

fn main() {
    let mut config = Config::default();
    load_config(&mut config);
    display_info();

    println!("Loading game...");

    let app = Application::new(config);
    unsafe {
        (*app.get()).run_loop();
    }
}