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

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct UnsafeCellWrapper<T: ?Sized>(UnsafeCell<T>);

impl<T> UnsafeCellWrapper<T> {
    pub fn new(field0: T) -> Self {
        Self(UnsafeCell::new(field0))
    }
}

impl<T: ?Sized> Deref for UnsafeCellWrapper<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for UnsafeCellWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<T: ?Sized> Send for UnsafeCellWrapper<T> {}

unsafe impl<T: ?Sized> Sync for UnsafeCellWrapper<T> {}