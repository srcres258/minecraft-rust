use std::slice::Iter;
use nalgebra_glm::Vec3;
use sfml::system::Vector3i;
use crate::physics::aabb::AABB;
use crate::util::mem::{uw_ref, uw_ref_mut, UWRef};
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
use crate::world::chunk::chunk_mesh_builder::ChunkMeshBuilder;
use crate::world::chunk::chunk::IChunk;
use crate::world::constants::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME};
use crate::world::world::World;

#[derive(Copy, Clone, Default)]
struct Layer {
    solid_block_count: i32
}

impl Layer {
    pub fn update(&mut self, c: ChunkBlock) {
        if c.data().is_opaque {
            self.solid_block_count -= 1;
        } else {
            self.solid_block_count += 1;
        }
    }

    pub fn all_solid(&self) -> bool {
        self.solid_block_count == CHUNK_AREA as i32
    }
}

pub struct ChunkSection {
    blocks: [ChunkBlock; CHUNK_VOLUME],
    layers: [Layer; CHUNK_SIZE],

    meshes: ChunkMeshCollection,
    aabb: AABB,
    location: Vector3i,

    world: UWRef<World>,

    has_mesh: bool,
    has_buffered_mesh: bool
}

impl ChunkSection {
    pub fn new(location: Vector3i, world: &World) -> Self {
        let mut result = Self {
            blocks: [ChunkBlock::default(); CHUNK_VOLUME],
            layers: [Layer::default(); CHUNK_SIZE],
            meshes: ChunkMeshCollection::default(),
            aabb: AABB::default(),
            location,
            world: uw_ref(world),
            has_mesh: false,
            has_buffered_mesh: false
        };
        result.aabb.update(Vec3::new(
            location.x as f32 * CHUNK_SIZE as f32,
            location.y as f32 * CHUNK_SIZE as f32,
            location.z as f32 * CHUNK_SIZE as f32
        ));
        result
    }

    pub fn location(&self) -> Vector3i {
        self.location
    }

    pub fn has_mesh(&self) -> bool {
        self.has_mesh
    }
    pub fn has_buffered(&self) -> bool {
        self.has_buffered_mesh
    }

    pub fn make_mesh(&mut self) {
        let mut self1 = uw_ref_mut(self);
        ChunkMeshBuilder::new(self, &mut self1.meshes).build_mesh();
        self.has_mesh = true;
        self.has_buffered_mesh = false;
    }
    pub fn buffer_mesh(&mut self) {
        self.meshes.solid_mesh.buffer_mesh();
        self.meshes.water_mesh.buffer_mesh();
        self.meshes.flora_mesh.buffer_mesh();
        self.has_buffered_mesh = true;
    }

    pub fn layer(&self, y: i32) -> &Layer {
        todo!()
    }
    pub fn adjacent(&self, dx: i32, dz: i32) -> &ChunkSection {
        todo!()
    }

    pub fn meshes(&self) -> &ChunkMeshCollection {
        &self.meshes
    }

    pub fn delete_meshes(&mut self) {
        if self.has_mesh {
            self.has_buffered_mesh = false;
            self.has_mesh = false;
            self.meshes.solid_mesh.delete_data();
            self.meshes.water_mesh.delete_data();
            self.meshes.flora_mesh.delete_data();
        }
    }

    pub fn iter(&self) -> Iter<'_, ChunkBlock> {
        self.blocks.iter()
    }
}

impl IChunk for ChunkSection {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        todo!()
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        todo!()
    }
}