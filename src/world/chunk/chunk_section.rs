use std::cell::RefCell;
use std::rc::Rc;
use sfml::system::Vector3i;
use crate::physics::aabb::AABB;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::world::World;
use crate::world::world_constants::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME};

#[derive(Copy, Clone, Default)]
pub struct Layer {
    solid_block_count: i32
}

pub struct ChunkSection {
    blocks: [ChunkBlock; CHUNK_VOLUME],
    layers: [Layer; CHUNK_SIZE],

    meshes: ChunkMeshCollection,
    aabb: AABB,
    location: Vector3i,

    p_world: Rc<RefCell<World>>,

    has_mesh: bool,
    has_buffered_mesh: bool
}

impl Layer {
    pub fn update(&mut self, c: ChunkBlock) {
        if c.get_data().is_opaque {
            self.solid_block_count -= 1;
        } else {
            self.solid_block_count += 1;
        }
    }

    pub fn is_all_solid(&self) -> bool {
        self.solid_block_count == CHUNK_AREA as _
    }
}