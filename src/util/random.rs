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

use std::sync::Mutex;
use std::time::SystemTime;
use lazy_static::lazy_static;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::StdRng;
use rand::Rng;
use rand_core::SeedableRng;

/// @brief Singleton class that increases randomness (such as with seeds)
pub struct RandomSingleton {
    random_engine: Mutex<StdRng>
}

pub struct Random {
    random_engine: Mutex<StdRng>
}

lazy_static! {
    static ref INSTANCE: RandomSingleton = RandomSingleton::new();
}

impl RandomSingleton {
    fn new() -> Self {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Self {
            random_engine: Mutex::new(StdRng::seed_from_u64(duration.as_secs()))
        }
    }

    pub fn get() -> &'static Self {
        &INSTANCE
    }

    pub fn int_in_range<T: SampleUniform>(&self, range: impl SampleRange<T>) -> T {
        self.random_engine.lock().unwrap().gen_range(range)
    }
}

impl Random {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            random_engine: Mutex::new(StdRng::seed_from_u64(seed))
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.random_engine = Mutex::new(StdRng::seed_from_u64(seed));
    }
    
    pub fn int_in_range<T: SampleUniform>(&self, range: impl SampleRange<T>) -> T {
        self.random_engine.lock().unwrap().gen_range(range)
    }
}

impl Default for Random {
    fn default() -> Self {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Self::new_with_seed(duration.as_secs())
    }
}