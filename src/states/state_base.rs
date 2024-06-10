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

use sfml::window::Event;
use crate::renderer::render_master::RenderMaster;

pub trait StateBase {
    fn handle_event(&mut self, event: Event);
    fn handle_input(&mut self);
    fn update(&mut self, delta_time: f32);
    fn render(&mut self, renderer: &mut RenderMaster);
    fn on_open(&mut self);
}