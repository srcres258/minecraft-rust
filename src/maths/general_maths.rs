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

fn smooth_step(edge0: f32, edge1: f32, mut x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    x = x * x * (3.0 - 2.0 * x);
    // Evaluate polynomial
    edge0 * x + edge1 * (1.0 - x)
}

pub fn smooth_interpolation(
    bottom_left: f32,
    top_left: f32,
    bottom_right: f32,
    top_right: f32,
    x_min: f32,
    x_max: f32,
    z_min: f32,
    z_max: f32,
    x: f32,
    z: f32
) -> f32 {
    let (width, height) = (x_max - x_min, z_max - z_min);
    let x_value = 1.0 - (x - x_min) / width;
    let z_value = 1.0 - (z - z_min) / height;

    let a = smooth_step(bottom_left, bottom_right, x_value);
    let b = smooth_step(top_left, top_right, x_value);

    smooth_step(a, b, z_value)
}