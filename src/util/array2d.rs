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

/*
TODO: There's some bug with the const generic parameters within Rust compiler.

When adding `const WIDTH: usize` to the generic list, the following error occurs:

error: unconstrained generic constant
 --> src/util/array2d.rs:3:12
  |
3 |     array: [T; WIDTH * WIDTH]
  |            ^^^^^^^^^^^^^^^^^^
  |
  = help: try adding a `where` bound using this expression: `where [(); WIDTH * WIDTH]:`

Hence the temporary solution is to switch the implementation to the version based on
heap memory allocation, using Vec to simulate an "array".
 */

#[derive(Clone)]
pub struct Array2D<T: Ord + Clone> {
    array: Vec<T>,
    width: usize
}

impl<T: Ord + Clone + Default> Array2D<T> {
    pub fn new(width: usize) -> Self {
        let mut result = Self {
            array: Vec::with_capacity(width * width),
            width
        };
        
        for _ in 0 .. width * width {
            result.array.push(T::default());
        }
        
        result
    }

    pub fn get(&self, x: usize, z: usize) -> &T {
        &self.array[x * self.width + z]
    }
    
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        &mut self.array[x * self.width + z]
    }
    
    pub fn get_max_value(&self) -> &T {
        self.array.iter().max().unwrap()
    }
    
    pub fn set_all(&mut self, val: T) {
        self.array.fill(val)
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
}