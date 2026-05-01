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

use nalgebra_glm::IVec3;
use sfml::system::Vector3i;

pub mod fps_counter;
pub mod array2d;
pub mod random;
pub mod file_util;
pub mod unsafe_cell_wrapper;

pub fn ivec3_to_vector3i(ivec3: IVec3) -> Vector3i {
    Vector3i::new(ivec3.x, ivec3.y, ivec3.z)
}

pub fn vector3i_to_ivec3(vector3i: Vector3i) -> IVec3 {
    IVec3::new(vector3i.x, vector3i.y, vector3i.z)
}