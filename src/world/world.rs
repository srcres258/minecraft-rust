use crate::camera::Camera;
use crate::config::Config;
use crate::event::world_event::IWorldEvent;
use crate::input::toggle_key::ToggleKey;
use crate::maths::vector2xz::VectorXZ;
use crate::player::player::Player;
use crate::renderer::render_master::RenderMaster;
use crate::util::mem::{uw_cell, uw_ref, uw_ref_mut, UWCell, UWRef, UWRefMut};
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_manager::ChunkManager;
use crate::world::chunk::chunk_section::ChunkSection;
use nalgebra_glm::{IVec3, Vec3};
use sfml::window::Key;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use sfml::system::Clock;
use crate::world::constants::CHUNK_SIZE;

pub struct World {
    chunk_manager: Option<UWCell<ChunkManager>>,

    events: Vec<Box<dyn IWorldEvent>>,
    chunk_updates: HashMap<IVec3, UWRefMut<ChunkSection>>,

    is_running: AtomicBool,
    chunk_load_threads: Vec<JoinHandle<()>>,

    // Mutex classes invoked to protect data from shared threads
    main_mutex: Mutex<()>,
    gen_mutex: Mutex<()>,

    load_distance: i32,
    render_distance: i32,

    player_spawn_point: Vec3,

    key: ToggleKey
}

impl World {
    pub fn new(camera: &Camera, config: Config, player: &mut Player) -> Self {
        let mut result = Self {
            chunk_manager: None,
            events: Vec::new(),
            chunk_updates: HashMap::new(),
            is_running: AtomicBool::new(true),
            chunk_load_threads: Vec::new(),
            main_mutex: Mutex::new(()),
            gen_mutex: Mutex::new(()),
            load_distance: 2,
            render_distance: config.render_distance,
            player_spawn_point: Vec3::default(),
            key: ToggleKey::new(Key::C)
        };

        result.chunk_manager = Some(uw_cell(ChunkManager::new(&result)));

        result.set_spawn_point();
        player.position = result.player_spawn_point;

        for _ in 0 .. 1 {
            let mut result1 = uw_ref_mut(&mut result);
            let camera1 = uw_ref(camera);
            thread::sleep(Duration::from_millis(200));
            result.chunk_load_threads.push(thread::spawn(move || {
                result1.load_chunks(camera1.deref());
            }));
        }

        result
    }

    pub fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        let bp = Self::block_xz(x, z);
        let chunk_pos = Self::chunk_xz(x, z);

        self.chunk_manager.as_ref().unwrap().get_mut().chunk(chunk_pos.x, chunk_pos.z)
            .block(bp.x, y, bp.z)
    }
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if y <= 0 {
            return;
        }

        let bp = Self::block_xz(x, z);
        let chunk_pos = Self::chunk_xz(x, z);

        self.chunk_manager.as_ref().unwrap().get_mut().chunk(chunk_pos.x, chunk_pos.z)
            .set_block(bp.x, y, bp.z, block);
    }

    pub fn update(&mut self, camera: &Camera) {
        if self.key.key_pressed() {
            let lock = self.main_mutex.lock().unwrap();
            self.chunk_manager.as_ref().unwrap().get_mut().delete_meshes();
            self.load_distance = 2;
        }

        let self1 = uw_ref(self);
        for event in self.events.iter_mut() {
            event.handle(self1.deref());
        }
        self.events.clear();

        self.update_chunks();
    }
    pub fn update_chunk(&mut self, block_x: i32, block_y: i32, block_z: i32) {
        let lock = self.main_mutex.lock().unwrap();

        let mut add_chunk_to_update_batch = |key: IVec3, section: &mut ChunkSection| {
            self.chunk_updates.insert(key, uw_ref_mut(section));
        };
        
        let chunk_pos = Self::chunk_xz(block_x, block_z);
        let chunk_section_y = block_y / CHUNK_SIZE as i32;
        
        let key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z);
        add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
            .chunk(chunk_pos.x, chunk_pos.z).section_mut(chunk_section_y));

        let section_block_xz = Self::block_xz(block_x, block_z);
        let section_block_y = block_y % CHUNK_SIZE as i32;

        if section_block_xz.x == 0 {
            let new_key = IVec3::new(chunk_pos.x - 1, chunk_section_y, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        } else if section_block_xz.x == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x + 1, chunk_section_y, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        }

        if section_block_y == 0 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y - 1, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        } else if section_block_y == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y + 1, chunk_pos.z);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        }

        if section_block_xz.z == 0 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z - 1);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        } else if section_block_xz.z == CHUNK_SIZE as i32 - 1 {
            let new_key = IVec3::new(chunk_pos.x, chunk_section_y, chunk_pos.z + 1);
            add_chunk_to_update_batch(key, self.chunk_manager.as_ref().unwrap().get_mut()
                .chunk(new_key.x, new_key.z).section_mut(new_key.y));
        }
    }

    pub fn render_world(&mut self, renderer: &RenderMaster, camera: &Camera) {
        let lock = self.main_mutex.lock().unwrap();
        //todo
    }

    pub fn chunk_manager(&self) -> &ChunkManager {
        self.chunk_manager.as_ref().unwrap().get()
    }
    pub fn chunk_manager_mut(&self) -> &mut ChunkManager {
        self.chunk_manager.as_ref().unwrap().get_mut()
    }

    pub fn block_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x % CHUNK_SIZE as i32, z % CHUNK_SIZE as i32)
    }
    pub fn chunk_xz(x: i32, z: i32) -> VectorXZ {
        VectorXZ::new(x / CHUNK_SIZE as i32, z / CHUNK_SIZE as i32)
    }

    pub fn add_event(&mut self, event: Box<dyn IWorldEvent>) {
        self.events.push(event);
    }

    fn load_chunks(&mut self, camera: &Camera) {
        // TODO (in C++): Optimize for chunkPositionU usage :thinking:

        while self.is_running.load(Ordering::Acquire) {
            let mut is_mesh_made = false;
            //todo
        }
    }
    fn update_chunks(&mut self) {
        let lock = self.main_mutex.lock().unwrap();
        for (_, s) in self.chunk_updates.iter_mut() {
            s.make_mesh();
        }
        self.chunk_updates.clear();
    }
    fn set_spawn_point(&mut self) {
        let timer = Clock::start();
        log::info!("Searching for spawn...");
        let mut attempts = 0;
        let mut chunk_x = -1;
        let mut chunk_z = -1;
        let mut block_x = 0;
        let mut block_y = 0;
        let mut block_z = 0;
        
        //todo
    }
}