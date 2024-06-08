extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use nalgebra_glm::IVec3;
use sfml::window::Key;
use crate::camera::Camera;
use crate::config::Config;
use crate::input::toggle_key::ToggleKey;
use crate::maths::vector2xz::VectorXZ;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_manager::ChunkManager;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::event::world_event::IWorldEvent;

/// @brief Massive class designed to hold multiple chunks, the player, and most game aspects.
pub struct World {
    chunk_manager: Option<ChunkManager>,

    events: Vec<Box<dyn IWorldEvent + Send>>,
    chunk_updates: HashMap<IVec3, Arc<Mutex<ChunkSection>>>,

    is_running: Mutex<bool>,
    chunk_load_threads: Vec<JoinHandle<()>>,

    load_distance: i32,
    render_distance: i32,

    player_spawn_point: glm::TVec3<f32>
}

const CHUNK_LOAD_THREADS_COUNT: usize = 1;

impl World {
    pub fn new(camera: Arc<Mutex<Camera>>, config: &Config, player: &mut Player) -> Arc<Mutex<Self>> {
        let result = Self {
            chunk_manager: None,
            events: Vec::new(),
            chunk_updates: HashMap::new(),
            is_running: Mutex::new(false),
            chunk_load_threads: Vec::new(),
            load_distance: 0,
            render_distance: config.render_distance,
            player_spawn_point: Default::default()
        };
        let result = Arc::new(Mutex::new(result));
        result.lock().unwrap().chunk_manager = Some(ChunkManager::new(result.clone()));

        result.lock().unwrap().set_spawn_point();
        player.wrapped_obj.borrow_mut().position = result.lock().unwrap().player_spawn_point;

        for _ in 0..CHUNK_LOAD_THREADS_COUNT {
            thread::sleep(Duration::from_millis(200));
            let obj = result.clone();
            let cam = camera.clone();
            result.lock().unwrap().chunk_load_threads.push(
                thread::spawn(move || {
                    obj.lock().unwrap().load_chunks(&cam.lock().unwrap());
                })
            );
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
    pub fn update(&mut self, camera: &Camera) {
        let mut key = ToggleKey::new(Key::C);

        if key.is_key_pressed() {
            self.chunk_manager.as_mut().unwrap().delete_meshes();
            self.load_distance = 2;
        }

        for event in self.events.iter() {
            event.handle(self);
        }
        self.events.clear();

        self.update_chunks();
    }

    pub fn update_chunk(&mut self, block_x: i32, block_y: i32, block_z: i32) {
        let add_chunk_to_update_batch = |key, section| {
            self.chunk_updates.insert(key, section);
        };
    }

    pub fn render_world(&self, master: &RenderMaster, camera: &Camera) {
        //todo
    }

    pub fn get_chunk_manager(&self) -> &ChunkManager {
        //todo
    }

    pub fn get_block_xz(x: i32, z: i32) -> VectorXZ {
        //todo
    }

    pub fn get_chunk_xz(x: i32, z: i32) -> VectorXZ {
        //todo
    }

    pub fn add_event(&mut self, event: Box<dyn IWorldEvent + Send>) {
        self.events.push(event);
    }

    fn load_chunks(&mut self, camera: &Camera) {
        //todo
    }

    fn update_chunks(&mut self) {
        //todo
    }

    fn set_spawn_point(&mut self) {
        //todo
    }
}

impl Drop for World {
    fn drop(&mut self) {
        *self.is_running.lock().unwrap() = false;
        while let Some(thread) = self.chunk_load_threads.pop() {
            thread.join().unwrap();
        }
    }
}