use std::cell::Cell;
use crate::camera::Camera;
use crate::config::Config;
use crate::event::world_event::IWorldEvent;
use crate::input::toggle_key::ToggleKey;
use crate::maths::vector2xz::VectorXZ;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::util::mem::{uw_cell, uw_ref, uw_ref_mut, UWBox, UWCell, UWRef, UWRefMut};
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_manager::ChunkManager;
use crate::world::chunk::chunk_section::ChunkSection;
use nalgebra_glm::{IVec3, Vec3};
use sfml::window::Key;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, ReentrantLock, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use sfml::system::Clock;
use crate::entity::Entity;
use crate::util::random::RandomSigleton;
use crate::util::singleton::Singleton;
use crate::world::constants::CHUNK_SIZE;

/// @brief Massive class designed to hold multiple chunks, the player, and most game aspects.
pub struct World {
    chunk_manager: UWCell<Option<Arc<UWBox<ChunkManager>>>>,

    events: Mutex<Vec<Box<dyn IWorldEvent + Send>>>,
    chunk_updates: Mutex<HashMap<IVec3, Arc<UWBox<ChunkSection>>>>,

    is_running: AtomicBool,
    chunk_load_threads: Mutex<Vec<JoinHandle<()>>>,

    load_distance: Mutex<i32>,
    render_distance: Mutex<i32>,

    player_spawn_point: Mutex<Vec3>
}

impl World {
    pub fn new(camera: Arc<Mutex<Camera>>, config: Config, player: &mut Player) -> Arc<Self> {
        let mut result = Self {
            chunk_manager: UWCell::new(None),
            events: Mutex::new(Vec::new()),
            chunk_updates: Mutex::new(HashMap::new()),
            is_running: AtomicBool::new(true),
            chunk_load_threads: Mutex::new(Vec::new()),
            load_distance: Mutex::new(2),
            render_distance: Mutex::new(config.render_distance),
            player_spawn_point: Mutex::new(Vec3::default())
        };

        let result = Arc::new(result);

        *result.chunk_manager.get_mut() = Some(ChunkManager::new(Arc::clone(&result)));

        result.set_spawn_point();
        player.position = *result.player_spawn_point.lock().unwrap();

        for _ in 0 .. 1 {
            let camera1 = Arc::clone(&camera);
            let mut result1 = Arc::clone(&result);
            thread::sleep(Duration::from_millis(200));
            result.chunk_load_threads.lock().unwrap().push(thread::spawn(move || {
                result1.load_chunks(camera1);
            }));
        }

        result
    }

    pub fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        let bp = Self::block_xz(x, z);
        let chunk_pos = Self::chunk_xz(x, z);

        self.chunk_manager.get().as_ref().unwrap().get_mut()
            .chunk(chunk_pos.x, chunk_pos.z)
            .block(bp.x, y, bp.z)
    }
    pub fn set_block(&self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if y <= 0 {
            return;
        }

        let bp = Self::block_xz(x, z);
        let chunk_pos = Self::chunk_xz(x, z);

        self.chunk_manager.get().as_ref().unwrap().get_mut()
            .chunk(chunk_pos.x, chunk_pos.z)
            .set_block(bp.x, y, bp.z, block);
    }

    pub fn update(&self) {
        let mut key = ToggleKey::new(Key::C);
        if key.key_pressed() {
            self.chunk_manager.get().as_ref().unwrap().get_mut().delete_meshes();
            *self.load_distance.lock().unwrap() = 2;
        }

        let mut events = self.events.lock().unwrap();
        for event in events.iter_mut() {
            event.handle(self);
        }
        events.clear();
        drop(events);

        self.update_chunks();
    }
    pub fn update_chunk(&self, block_x: i32, block_y: i32, block_z: i32) {
        let mut add_chunk_to_update_batch = |key: IVec3, section: Arc<UWBox<ChunkSection>>| {
            self.chunk_updates.lock().unwrap().insert(key, section);
        };
        
        let chunk_pos = Self::chunk_xz(block_x, block_z);
        let chunk_section_y = block_y / CHUNK_SIZE as i32;
        
        let key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z);
        add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
            .chunk(chunk_pos.x, chunk_pos.z).section(chunk_section_y));

        let section_block_xz = Self::block_xz(block_x, block_z);
        let section_block_y = block_y % CHUNK_SIZE as i32;

        if section_block_xz.x == 0 {
            let new_key = IVec3::new(chunk_pos.x - 1, chunk_section_y, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        } else if section_block_xz.x == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x + 1, chunk_section_y, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        }

        if section_block_y == 0 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y - 1, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        } else if section_block_y == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y + 1, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        }

        if section_block_xz.z == 0 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z - 1);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        } else if section_block_xz.z == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z + 1);
            add_chunk_to_update_batch(key, self.chunk_manager.get().as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section(new_key.y));
        }
    }

    pub fn render_world(&self, renderer: &mut RenderMaster, camera: Arc<Mutex<Camera>>) {
        renderer.draw_sky();

        let mut chunk_manager = self.chunk_manager.get().as_ref().unwrap().get_mut();
        let chunk_map = chunk_manager.chunks_mut();
        let mut keys_to_be_removed: Vec<VectorXZ> = Vec::new();
        for (key, chunk) in chunk_map.iter_mut() {
            let camera_obj = camera.lock().unwrap();
            let camera_x = camera_obj.position().x as i32;
            let camera_z = camera_obj.position().z as i32;
            drop(camera_obj);

            let render_distance = self.render_distance.lock().unwrap();
            let min_x = camera_x / CHUNK_SIZE as i32 - *render_distance;
            let min_z = camera_z / CHUNK_SIZE as i32 - *render_distance;
            let max_x = camera_x / CHUNK_SIZE as i32 + *render_distance;
            let max_z = camera_z / CHUNK_SIZE as i32 + *render_distance;
            drop(render_distance);

            let location = chunk.location();

            if min_x > location.x || min_z > location.y ||
                max_z < location.y || max_x < location.x {
                keys_to_be_removed.push(*key);
                continue;
            } else {
                chunk.draw_chunks(renderer, &camera.lock().unwrap());
            }
        }
        for key in keys_to_be_removed.iter() {
            chunk_map.remove(key);
        }
    }

    pub fn chunk_manager(&self) -> Arc<UWBox<ChunkManager>> {
        Arc::clone(self.chunk_manager.get().as_ref().unwrap())
    }

    pub fn block_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x % CHUNK_SIZE as i32, z % CHUNK_SIZE as i32)
    }
    pub fn chunk_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x / CHUNK_SIZE as i32, z / CHUNK_SIZE as i32)
    }

    pub fn add_event(&self, event: Box<dyn IWorldEvent + Send>) {
        self.events.lock().unwrap().push(event);
    }

    fn load_chunks(&self, camera: Arc<Mutex<Camera>>) {
        // TODO (in C++): Optimize for chunkPositionU usage :thinking:

        while self.is_running.load(Ordering::Acquire) {
            let mut is_mesh_made = false;
            let camera_obj = camera.lock().unwrap();
            let camera_x = camera_obj.position().x as i32 / CHUNK_SIZE as i32;
            let camera_z = camera_obj.position().x as i32 / CHUNK_SIZE as i32;
            drop(camera_obj);

            'inner: for i in 0 .. *self.load_distance.lock().unwrap() {
                thread::sleep(Duration::from_millis(1));
                let min_x = (camera_x - i).max(0);
                let min_z = (camera_z - i).max(0);
                let max_x = camera_x + i;
                let max_z = camera_z + i;

                for x in min_x .. max_x {
                    for z in min_z .. max_z {
                        is_mesh_made = self.chunk_manager.get().as_ref().unwrap().get_mut()
                            .make_mesh(x, z, Arc::clone(&camera));
                    }
                }

                if is_mesh_made {
                    break 'inner;
                }
            }

            if !is_mesh_made {
                *self.load_distance.lock().unwrap() += 1;
            }
            if *self.load_distance.lock().unwrap() >= *self.render_distance.lock().unwrap() {
                *self.load_distance.lock().unwrap() = 2;
            }
        }
    }
    fn update_chunks(&self) {
        let mut chunk_updates = self.chunk_updates.lock().unwrap();
        for (_, s) in chunk_updates.iter_mut() {
            s.get_mut().make_mesh();
        }
        chunk_updates.clear();
    }
    fn set_spawn_point(&self) {
        let timer = Clock::start();
        log::info!("Searching for spawn...");
        let mut attempts = 0;
        let mut chunk_x = -1;
        let mut chunk_z = -1;
        let mut block_x = 0;
        let mut block_y = 0;
        let mut block_z = 0;

        let h = self.chunk_manager.get().as_ref().unwrap().get_mut().terrain_generator().minimum_spawn_height();

        while block_y <= h {
            self.chunk_manager.get().as_ref().unwrap().get_mut().unload_chunk(chunk_x, chunk_z);

            chunk_x = RandomSigleton::get().i32_in_range(100, 200);
            chunk_z = RandomSigleton::get().i32_in_range(100, 200);
            block_x = RandomSigleton::get().i32_in_range(0, 15);
            block_z = RandomSigleton::get().i32_in_range(0, 15);

            self.chunk_manager.get().as_ref().unwrap().get_mut().load_chunk(chunk_x, chunk_z);
            block_y = self.chunk_manager.get().as_ref().unwrap().get_mut().chunk(chunk_x, chunk_z).height_at(block_x, block_z);
            attempts += 1;
        }

        let world_x = chunk_x * CHUNK_SIZE as i32 + block_x;
        let world_z = chunk_z * CHUNK_SIZE as i32 + block_z;

        *self.player_spawn_point.lock().unwrap() = Vec3::new(world_x as _, block_y as _, world_z as _);

        for x in world_x - 1 ..= world_x + 1 {
            for z in world_z - 1 ..= world_z + 1 {
                self.chunk_manager.get().as_ref().unwrap().get_mut().load_chunk(x, z);
            }
        }

        log::info!("Spawn found! Attempts: {}", attempts);
        log::info!("Time Taken: {} seconds", timer.elapsed_time().as_seconds());
    }
}

impl Drop for World {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Release);
        let mut chunk_load_threads = self.chunk_load_threads.lock().unwrap();
        while let Some(thread) = chunk_load_threads.pop() {
            thread.join().unwrap();
        }
    }
}