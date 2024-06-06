extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use sfml::system::Vector3i;
use crate::camera::Camera;
use crate::config::Config;
use crate::maths::vector2xz::VectorXZ;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_manager::ChunkManager;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::event::world_event::IWorldEvent;

/// @brief Massive class designed to hold multiple chunks, the player, and most game aspects.
pub struct World {
    chunk_manager: ChunkManager,

    events: Vec<Box<dyn IWorldEvent>>,
    chunk_updates: HashMap<Vector3i, Arc<Mutex<ChunkSection>>>,

    is_running: Mutex<bool>,
    chunk_load_threads: Vec<Thread>,

    load_distance: i32,
    render_distance: i32,

    player_spawn_point: glm::TVec3<f32>
}

impl World {
    pub fn new(camera: &Camera, config: &Config, player: &Player) -> Self {
        //todo
    }

    // world coords into chunk column coords
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        //todo
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        //todo
    }

    pub fn update(&mut self, camera: &Camera) {
        //todo
    }

    pub fn update_chunk(&mut self, block_x: i32, block_y: i32, block_z: i32) {
        //todo
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

    pub fn add_event(&mut self, event: Box<dyn IWorldEvent>) {
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
        //todo
    }
}