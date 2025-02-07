use std::ops::Deref;
use gl::types::GLfloat;
use sfml::system::{Clock, Vector2i, Vector3i};
use crate::util::mem::{uw_ref, uw_ref_mut, uw_ref_ptr, UWRef, UWRefMut};
use crate::world::block::block_data::{BlockDataHolder, BlockMeshType, BlockShaderType};
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk_mesh::{ChunkMesh, ChunkMeshCollection};
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::chunk::chunk::IChunk;
use crate::world::constants::{CHUNK_SIZE, CHUNK_VOLUME};

const FRONT_FACE: [GLfloat; 12] = [0., 0., 1., 1., 0., 1., 1., 1., 1., 0., 1., 1.];
const BACK_FACE: [GLfloat; 12] = [1., 0., 0., 0., 0., 0., 0., 1., 0., 1., 1., 0.];
const LEFT_FACE: [GLfloat; 12] = [0., 0., 0., 0., 0., 1., 0., 1., 1., 0., 1., 0.];
const RIGHT_FACE: [GLfloat; 12] = [1., 0., 1., 1., 0., 0., 1., 1., 0., 1., 1., 1.];
const TOP_FACE: [GLfloat; 12] = [0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 1., 0.];
const BOTTOM_FACE: [GLfloat; 12] = [0., 0., 0., 1., 0., 0., 1., 0., 1., 0., 0., 1.];
const X_FACE_1: [GLfloat; 12] = [0., 0., 0., 1., 0., 1., 1., 1., 1., 0., 1., 0.];
const X_FACE_2: [GLfloat; 12] = [0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1., ];

const LIGHT_TOP: GLfloat = 1.0;
const LIGHT_X: GLfloat = 0.8;
const LIGHT_Z: GLfloat = 0.6;
const LIGHT_BOT: GLfloat = 0.4;

#[derive(Default)]
struct AdjacentBlockPositions {
    up: Vector3i,
    down: Vector3i,
    left: Vector3i,
    right: Vector3i,
    front: Vector3i,
    back: Vector3i
}

impl AdjacentBlockPositions {
    pub fn update(&mut self, x: i32, y: i32, z: i32) {
        self.up = Vector3i::new(x, y + 1, z);
        self.down = Vector3i::new(x, y - 1, z);
        self.left = Vector3i::new(x - 1, y, z);
        self.right = Vector3i::new(x + 1, y, z);
        self.front = Vector3i::new(x, y, z + 1);
        self.back = Vector3i::new(x, y, z - 1);
    }
}

#[derive(Default)]
pub struct ChunkMeshBuilder {
    chunk: Option<UWRef<ChunkSection>>,
    meshes: Option<UWRefMut<ChunkMeshCollection>>,
    active_mesh: Option<UWRefMut<ChunkMesh>>,
    block_data: Option<UWRef<BlockDataHolder>>,

    faces: i32
}

impl ChunkMeshBuilder {
    pub fn new(chunk: &ChunkSection, meshes: &mut ChunkMeshCollection) -> Self {
        let mut result = Self::default();
        result.chunk = Some(uw_ref(chunk));
        result.meshes = Some(uw_ref_mut(meshes));
        result
    }

    pub fn build_mesh(&mut self) {
        let mut directions = AdjacentBlockPositions::default();
        let self1 = uw_ref(self);
        let mut self2 = uw_ref_mut(self);
        let mut block_iter = self1.chunk.as_ref().unwrap().iter();
        self.faces = 0;
        for i in 0 .. CHUNK_VOLUME {
            let x = (i % CHUNK_SIZE) as i32;
            let y = (i / (CHUNK_SIZE * CHUNK_SIZE)) as i32;
            let z = ((i / CHUNK_SIZE) % CHUNK_SIZE) as i32;

            if !self.should_make_layer(y) {
                continue;
            }

            let block = block_iter.next().unwrap();

            let position = Vector3i::new(x, y, z);
            self.set_active_mesh(*block);

            if block.id == BlockId::Air as _ {
                continue
            }

            self.block_data = Some(uw_ref(block.data()));
            let data = self.block_data.as_ref().unwrap().clone();

            if data.mesh_type == BlockMeshType::X {
                self.add_x_block_to_mesh(data.tex_top_coord, position);
                continue;
            }

            directions.update(x, y, z);

            // Up / Down
            if self.chunk.as_ref().unwrap().location().y != 0 || y != 0 {
                self2.try_add_face_to_mesh(
                    BOTTOM_FACE,
                    data.tex_bottom_coord,
                    position,
                    directions.down,
                    LIGHT_BOT
                )
            }
            self2.try_add_face_to_mesh(
                TOP_FACE,
                data.tex_top_coord,
                position,
                directions.up,
                LIGHT_TOP
            );

            // Left / Right
            self2.try_add_face_to_mesh(
                LEFT_FACE,
                data.tex_side_coord,
                position,
                directions.left,
                LIGHT_X
            );
            self2.try_add_face_to_mesh(
                RIGHT_FACE,
                data.tex_side_coord,
                position,
                directions.right,
                LIGHT_X
            );

            // Front / Back
            self2.try_add_face_to_mesh(
                FRONT_FACE,
                data.tex_side_coord,
                position,
                directions.front,
                LIGHT_Z
            );
            self2.try_add_face_to_mesh(
                BACK_FACE,
                data.tex_side_coord,
                position,
                directions.back,
                LIGHT_Z
            );
        }
    }

    fn set_active_mesh(&mut self, block: ChunkBlock) {
        match block.data().shader_type {
            BlockShaderType::Chunk => {
                self.active_mesh = Some(uw_ref_mut(&mut self.meshes.as_mut().unwrap().solid_mesh));
            }
            BlockShaderType::Liquid => {
                self.active_mesh = Some(uw_ref_mut(&mut self.meshes.as_mut().unwrap().water_mesh));
            }
            BlockShaderType::Flora => {
                self.active_mesh = Some(uw_ref_mut(&mut self.meshes.as_mut().unwrap().flora_mesh));
            }
        }
    }

    fn add_x_block_to_mesh(
        &mut self,
        texture_coords: Vector2i,
        block_position: Vector3i
    ) {
        self.faces += 1;
        let tex_coords = BlockDatabase::get().texture_atlas.texture(texture_coords);

        self.active_mesh.as_mut().unwrap().add_face(
            X_FACE_1,
            tex_coords,
            self.chunk.as_ref().unwrap().location(),
            block_position,
            LIGHT_X
        );
        self.active_mesh.as_mut().unwrap().add_face(
            X_FACE_2,
            tex_coords,
            self.chunk.as_ref().unwrap().location(),
            block_position,
            LIGHT_X
        );
    }

    fn try_add_face_to_mesh(
        &mut self,
        block_face: [GLfloat; 12],
        texture_coords: Vector2i,
        block_position: Vector3i,
        block_facing: Vector3i,
        cardinal_light: GLfloat
    ) {
        if self.should_make_face(block_facing, self.block_data.as_ref().unwrap()) {
            self.faces += 1;
            let tex_coords = BlockDatabase::get().texture_atlas.texture(texture_coords);

            self.active_mesh.as_mut().unwrap().add_face(
                block_face,
                tex_coords,
                self.chunk.as_ref().unwrap().location(),
                block_position,
                cardinal_light
            );
        }
    }

    fn should_make_face(
        &self,
        adj_block: Vector3i,
        block_data: &BlockDataHolder
    ) -> bool {
        let block = self.chunk.as_ref().unwrap().block(adj_block.x, adj_block.y, adj_block.z);
        let data = block.data();
        
        if block.id == BlockId::Air as _ {
            return true;
        } else if !data.is_opaque && data.id != self.block_data.as_ref().unwrap().id {
            return true;
        }
        
        false
    }

    fn should_make_layer(&mut self, y: i32) -> bool {
        let adj_is_solid = |dx: i32, dz: i32| {
            let sect = self.chunk.as_ref().unwrap().adjacent(dx, dz);
            sect.layer(y).all_solid()
        };
        
        !self.chunk.as_ref().unwrap().layer(y).all_solid() || 
            !self.chunk.as_ref().unwrap().layer(y + 1).all_solid() || 
            !self.chunk.as_ref().unwrap().layer(y - 1).all_solid() ||
            
            adj_is_solid(1, 0) ||
            adj_is_solid(0, 1) ||
            adj_is_solid(-1, 0) ||
            adj_is_solid(0, -1)
    }
}