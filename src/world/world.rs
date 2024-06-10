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

extern crate nalgebra_glm as glm;

use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use nalgebra_glm::IVec3;
use sfml::system::{Clock, Vector3i};
use sfml::window::Key;
use crate::camera::Camera;
use crate::config::Config;
use crate::input::toggle_key::ToggleKey;
use crate::maths::vector2xz::VectorXZ;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::util;
use crate::util::random::RandomSingleton;
use crate::util::unsafe_cell_wrapper::UnsafeCellWrapper;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_manager::ChunkManager;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::event::world_event::IWorldEvent;
use crate::world::world_constants::CHUNK_SIZE;

/// @brief Massive class designed to hold multiple chunks, the player, and most game aspects.
pub struct World {
    chunk_manager: Option<ChunkManager>,

    events: Vec<Box<dyn IWorldEvent + Send>>,
    chunk_updates: HashMap<IVec3, *mut ChunkSection>,

    is_running: AtomicBool,
    chunk_load_threads: Vec<JoinHandle<()>>,

    main_mutex: Mutex<()>,

    load_distance: i32,
    render_distance: i32,

    player_spawn_point: glm::TVec3<f32>
}

const CHUNK_LOAD_THREADS_COUNT: usize = 1;

impl World {
    pub fn new(
        camera: Arc<UnsafeCellWrapper<Camera>>,
        config: &Config,
        player: &mut Player
    ) -> Arc<UnsafeCellWrapper<Self>> {
        let result = Self {
            chunk_manager: None,
            events: Vec::new(),
            chunk_updates: HashMap::new(),
            is_running: AtomicBool::new(true),
            chunk_load_threads: Vec::new(),
            main_mutex: Mutex::new(()),
            load_distance: 0,
            render_distance: config.render_distance,
            player_spawn_point: Default::default()
        };
        let result = Arc::new(UnsafeCellWrapper::new(result));
        unsafe {
            (*result.get()).chunk_manager = Some(ChunkManager::new(Arc::clone(&result)));

            (*result.get()).set_spawn_point();
            player.position = (*result.get()).player_spawn_point;

            for _ in 0..CHUNK_LOAD_THREADS_COUNT {
                thread::sleep(Duration::from_millis(200));
                let obj = Arc::clone(&result);
                let cam = Arc::clone(&camera);
                (*result.get()).chunk_load_threads.push(
                    thread::spawn(move || {
                        (*obj.get()).load_chunks(&*cam.get());
                    })
                );
            }
        }

        result
    }

    // world coords into chunk column coords
    pub fn get_block(&mut self, x: i32, y: i32, z: i32) -> ChunkBlock {
        let bp = Self::get_block_xz(x, z);
        let chunk_position = Self::get_chunk_xz(x, z);

        self.chunk_manager.as_mut().unwrap().get_chunk(chunk_position.x, chunk_position.z)
            .get_block(bp.x, y, bp.z)
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if y <= 0 {
            return;
        }

        let bp = Self::get_block_xz(x, z);
        let chunk_position = Self::get_chunk_xz(x, z);

        self.chunk_manager.as_mut().unwrap().get_chunk_mut(chunk_position.x, chunk_position.z)
            .set_block(bp.x, y, bp.z, block);
    }

    // loads chunks
    // make chunk meshes
    pub fn update(this: &UnsafeCell<Self>, _camera: &Camera) {
        let mut key = ToggleKey::new(Key::C);

        unsafe {
            if key.is_key_pressed() {
                let lock = (*this.get()).main_mutex.lock().unwrap();
                (*this.get()).chunk_manager.as_mut().unwrap().delete_meshes();
                (*this.get()).load_distance = 2;
                drop(lock);
            }

            for event in (*this.get()).events.iter() {
                event.handle(&mut *this.get());
            }
            (*this.get()).events.clear();

            (*this.get()).update_chunks();
        }
    }

    pub fn update_chunk(&mut self, block_x: i32, block_y: i32, block_z: i32) {
        let lock = self.main_mutex.lock().unwrap();

        let mut add_chunk_to_update_batch = |key, section| {
            self.chunk_updates.insert(key, section);
        };

        let chunk_position = Self::get_chunk_xz(block_x, block_z);
        let chunk_section_y = block_y / CHUNK_SIZE as i32;

        let key = Vector3i::new(
            chunk_position.x, chunk_section_y, chunk_position.z
        );
        add_chunk_to_update_batch(
            util::vector3i_to_ivec3(key),
            self.chunk_manager.as_mut()
                .unwrap()
                .get_chunk_mut(chunk_position.x, chunk_position.z)
                .get_section_mut(chunk_section_y)
        );

        let section_block_xz = Self::get_block_xz(block_x, block_z);
        let section_block_y = block_y % CHUNK_SIZE as i32;

        if section_block_xz.x == 0 {
            let new_key = Vector3i::new(
                chunk_position.x - 1, chunk_section_y, chunk_position.z
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        } else if section_block_xz.x == CHUNK_SIZE as i32 - 1 {
            let new_key = Vector3i::new(
                chunk_position.x + 1, chunk_section_y, chunk_position.z
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        }

        if section_block_y == 0 {
            let new_key = Vector3i::new(
                chunk_position.x, chunk_section_y - 1, chunk_position.z
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        } else if section_block_y == CHUNK_SIZE as i32 - 1 {
            let new_key = Vector3i::new(
                chunk_position.x, chunk_section_y + 1, chunk_position.z
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        }

        if section_block_xz.z == 0 {
            let new_key = Vector3i::new(
                chunk_position.x, chunk_section_y, chunk_position.z - 1
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        } else if section_block_xz.z == CHUNK_SIZE as i32 - 1 {
            let new_key = Vector3i::new(
                chunk_position.x, chunk_section_y, chunk_position.z + 1
            );
            add_chunk_to_update_batch(
                util::vector3i_to_ivec3(new_key),
                self.chunk_manager.as_mut()
                    .unwrap()
                    .get_chunk_mut(new_key.x, new_key.z)
                    .get_section_mut(new_key.y)
            );
        }

        drop(lock);
    }

    pub fn render_world(&mut self, renderer: &mut RenderMaster, camera: &Camera) {
        let lock = self.main_mutex.lock().unwrap();

        renderer.draw_sky();

        let chunk_map = self.chunk_manager.as_mut().unwrap().get_chunks_mut();
        let mut keys_to_remove: HashSet<VectorXZ> = HashSet::new();
        for (key, chunk) in chunk_map.iter_mut() {
            let camera_x = camera.position.x as i32;
            let camera_z = camera.position.z as i32;

            let min_x = (camera_x / CHUNK_SIZE as i32) - self.render_distance;
            let min_z = (camera_z / CHUNK_SIZE as i32) - self.render_distance;
            let max_x = (camera_x / CHUNK_SIZE as i32) + self.render_distance;
            let max_z = (camera_z / CHUNK_SIZE as i32) + self.render_distance;

            let location = chunk.get_location();

            if min_x > location.x || min_z > location.y ||
                max_z < location.y || max_x < location.x {
                keys_to_remove.insert(*key);
            } else {
                chunk.draw_chunks(renderer, camera);
            }
        }
        for key in keys_to_remove.iter() {
            chunk_map.remove(key);
        }

        drop(lock);
    }

    pub fn get_chunk_manager(&self) -> &ChunkManager {
        self.chunk_manager.as_ref().unwrap()
    }

    pub fn get_chunk_manager_mut(&mut self) -> &mut ChunkManager {
        self.chunk_manager.as_mut().unwrap()
    }

    pub fn get_block_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x % CHUNK_SIZE as i32, z % CHUNK_SIZE as i32)
    }

    pub fn get_chunk_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x / CHUNK_SIZE as i32, z / CHUNK_SIZE as i32)
    }

    pub fn add_event(&mut self, event: Box<dyn IWorldEvent + Send>) {
        self.events.push(event);
    }

    fn load_chunks(&mut self, camera: &Camera) {
        while self.is_running.load(Ordering::Acquire) {
            let mut is_mesh_made = false;
            let camera_x = camera.position.x as i32 / CHUNK_SIZE as i32;
            let camera_z = camera.position.z as i32 / CHUNK_SIZE as i32;

            'inner: for i in 0..self.load_distance {
                thread::sleep(Duration::from_millis(1));
                let min_x = (camera_x - i).max(0);
                let min_z = (camera_z - i).max(0);
                let max_x = camera_x + i;
                let max_z = camera_z + i;

                for x in min_x..max_x {
                    for z in min_z..max_z {
                        let lock = self.main_mutex.lock().unwrap();
                        is_mesh_made = self.chunk_manager.as_mut().unwrap().make_mesh(x, z, &camera);
                        drop(lock);
                    }
                }

                if is_mesh_made {
                    break 'inner;
                }
            }

            if !is_mesh_made {
                self.load_distance += 1;
            }
            if self.load_distance >= self.render_distance {
                self.load_distance = 2;
            }
        }
    }

    fn update_chunks(&mut self) {
        let lock = self.main_mutex.lock().unwrap();
        for c in self.chunk_updates.iter() {
            unsafe {
                let s = &mut **c.1;
                s.make_mesh();
            }
        }
        drop(lock);
    }

    fn set_spawn_point(&mut self) {
        let timer = Clock::start();
        log::info!("Searching for spawn...");
        let mut attempts = 0;
        let mut chunk_x = -1;
        let mut chunk_z = -1;
        let mut block_x = 0;
        let mut block_z = 0;
        let mut block_y = 0;
        
        let h = self.chunk_manager.as_ref().unwrap()
            .get_terrain_generator()
            .get_minimum_spawn_height();
        
        while block_y <= h {
            self.chunk_manager.as_mut().unwrap().unload_chunk(chunk_x, chunk_z);
            
            chunk_x = RandomSingleton::get().int_in_range(100..=200);
            chunk_z = RandomSingleton::get().int_in_range(100..=200);
            block_x = RandomSingleton::get().int_in_range(0..=15);
            block_z = RandomSingleton::get().int_in_range(0..=15);
            
            self.chunk_manager.as_mut().unwrap().load_chunk(chunk_x, chunk_z);
            block_y = self.chunk_manager.as_mut().unwrap()
                .get_chunk(chunk_x, chunk_z).get_height_at(block_x, block_z);
            attempts += 1;
        }
        
        let world_x = chunk_x * CHUNK_SIZE as i32 + block_x;
        let world_z = chunk_z * CHUNK_SIZE as i32 + block_z;
        
        self.player_spawn_point = glm::vec3(world_x as _, block_y as _, world_z as _);
        
        for x in world_x - 1 ..= world_x + 1 {
            for z in world_z - 1 ..= world_z + 1 {
                let lock = self.main_mutex.lock().unwrap();
                self.chunk_manager.as_mut().unwrap().load_chunk(x, z);
                drop(lock);
            }
        }

        log::info!(
            "Spawn found! Attempts: {} Time Taken: {} seconds",
            attempts,
            timer.elapsed_time().as_seconds()
        );
    }
}

impl Drop for World {
    fn drop(&mut self) {
        *self.is_running.get_mut() = false;
        while let Some(thread) = self.chunk_load_threads.pop() {
            thread.join().unwrap();
        }
    }
}