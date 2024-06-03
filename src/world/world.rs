extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use sfml::system::Vector3i;

/// @brief Massive class designed to hold multiple chunks, the player, and most game aspects.
pub struct World {
    chunk_manager: ChunkManager,

    events: Vec<Box<dyn IWorldEvent>>,
    chunk_updates: HashMap<Vector3i, Arc<ChunkSection>>,

    is_running: Mutex<bool>,
    chunk_load_threads: Vec<Thread>,

    load_distance: i32,
    render_distance: i32,

    player_spawn_point: glm::TVec3<f32>
}