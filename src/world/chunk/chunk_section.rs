extern crate nalgebra_glm as glm;

use std::cell::RefCell;
use std::slice::Iter;
use std::rc::Rc;
use sfml::system::Vector3i;
use crate::physics::aabb::AABB;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
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

impl ChunkSection {
    pub fn new(
        location: Vector3i,
        world: Rc<RefCell<World>>
    ) -> Self {
        let mut result = Self {
            blocks: Default::default(),
            layers: Default::default(),
            meshes: Default::default(),
            aabb: AABB::new(&glm::vec3(CHUNK_SIZE as _, CHUNK_SIZE as _, CHUNK_SIZE as _)),
            location,
            p_world: world,
            has_mesh: false,
            has_buffered_mesh: false
        };
        result.aabb.update(&glm::vec3(
            (location.x * CHUNK_SIZE as i32) as _,
            (location.y * CHUNK_SIZE as i32) as _,
            (location.z * CHUNK_SIZE as i32) as _
        ));
        result
    }
    
    pub fn get_location(&self) -> Vector3i {
        self.location
    }
    
    pub fn has_mesh(&self) -> bool {
        self.has_mesh
    }
    
    pub fn has_buffered(&self) -> bool {
        self.has_buffered_mesh
    }
    
    pub fn make_mesh(&mut self) {
        //todo
    }
    
    pub fn buffer_mesh(&mut self) {
        self.meshes.solid_mesh.buffer_mesh();
        self.meshes.water_mesh.buffer_mesh();
        self.meshes.flora_mesh.buffer_mesh();
        self.has_buffered_mesh = true;
    }
    
    pub fn get_layer(&self, y: i32) -> &Layer {
        //todo
    }
    
    pub fn get_adjacent(&mut self, dx: i32, dz: i32) -> &ChunkSection {
        //todo
    }
    
    pub fn get_meshes(&self) -> &ChunkMeshCollection {
        &self.meshes
    }
    
    pub fn delete_meshes(&mut self) {
        //todo
    }
    
    pub fn iter(&self) -> Iter<'_, ChunkBlock> {
        self.blocks.iter()
    }

    fn to_world_position(&self, x: i32, y: i32, z: i32) -> Vector3i {
        Vector3i::new(
            self.location.x * CHUNK_SIZE as i32 + x,
            self.location.y * CHUNK_SIZE as i32 + y,
            self.location.z * CHUNK_SIZE as i32 + z
        )
    }

    fn out_of_bounds(value: i32) -> bool {
        value >= CHUNK_SIZE as i32 || value < 0
    }

    fn get_index(x: i32, y: i32, z: i32) -> i32 {
        y * CHUNK_AREA as i32 + z * CHUNK_SIZE as i32 + x
    }
}

impl IChunk for ChunkSection {
    fn get_block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            return todo!();
        }

        self.blocks[Self::get_index(x, y, z)]
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            //todo
            return;
        }

        self.layers[y].update(block);

        self.blocks[Self::get_index(x, y, z)] = block;
    }
}