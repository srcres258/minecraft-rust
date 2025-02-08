use crate::physics::aabb::AABB;
use crate::util::mem::{uw_ref_mut, UWRefMut};
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
use crate::world::chunk::chunk_mesh_builder::ChunkMeshBuilder;
use crate::world::constants::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME};
use crate::world::world::World;
use nalgebra_glm::Vec3;
use sfml::system::Vector3i;
use std::slice::Iter;

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

    world: UWRefMut<World>,

    has_mesh: bool,
    has_buffered_mesh: bool
}

impl ChunkSection {
    pub fn new(location: Vector3i, world: &mut World) -> Self {
        let mut result = Self {
            blocks: [ChunkBlock::default(); CHUNK_VOLUME],
            layers: [Layer::default(); CHUNK_SIZE],
            meshes: ChunkMeshCollection::default(),
            aabb: AABB::default(),
            location,
            world: uw_ref_mut(world),
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
        const CS: i32 = CHUNK_SIZE as _;
        match y {
            -1 => self.world.chunk_manager_mut()
                .chunk(self.location.x, self.location.z)
                .section(self.location.y - 1)
                .layer(CS - 1),
            CS => self.world.chunk_manager_mut()
                .chunk(self.location.x, self.location.z)
                .section(self.location.y + 1)
                .layer(0),
            _ => &self.layers[y as usize]
        }
    }
    pub fn layer_mut(&mut self, y: i32) -> &mut Layer {
        const CS: i32 = CHUNK_SIZE as _;
        match y {
            -1 => self.world.chunk_manager_mut()
                .chunk(self.location.x, self.location.z)
                .section_mut(self.location.y - 1)
                .layer_mut(CS - 1),
            CS => self.world.chunk_manager_mut()
                .chunk(self.location.x, self.location.z)
                .section_mut(self.location.y + 1)
                .layer_mut(0),
            _ => &mut self.layers[y as usize]
        }
    }
    pub fn adjacent(&self, dx: i32, dz: i32) -> &ChunkSection {
        let new_x = self.location.x + dx;
        let new_z = self.location.z + dz;
        
        self.world.chunk_manager_mut()
            .chunk(new_x, new_z)
            .section(self.location.y)
    }
    pub fn adjacent_mut(&self, dx: i32, dz: i32) -> &mut ChunkSection {
        let new_x = self.location.x + dx;
        let new_z = self.location.z + dz;
        
        self.world.chunk_manager_mut()
            .chunk(new_x, new_z)
            .section_mut(self.location.y)
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
    
    pub fn aabb(&self) -> AABB {
        self.aabb
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
    fn index(x: i32, y: i32, z: i32) -> usize {
        y as usize * CHUNK_AREA + z as usize * CHUNK_SIZE + x as usize
    }
}

impl IChunk for ChunkSection {
    fn block(&self, x: i32, y: i32, z: i32) -> ChunkBlock {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            self.world.block(location.x, location.y, location.z)
        } else {
            self.blocks[Self::index(x, y, z)]
        }
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: ChunkBlock) {
        if Self::out_of_bounds(x) || Self::out_of_bounds(y) || Self::out_of_bounds(z) {
            let location = self.to_world_position(x, y, z);
            self.world.set_block(location.x, location.y, location.z, block);
        } else {
            self.layers[y as usize].update(block);
            self.blocks[Self::index(x, y, z)] = block;
        }
    }
}