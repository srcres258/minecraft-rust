use gl::types::GLfloat;
use sfml::system::{Vector2i, Vector3i};
use crate::world::block::block_data::{BlockDataHolder, BlockMeshType, BlockShaderType};
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_mesh::{ChunkMesh, ChunkMeshCollection};
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::world_constants::{CHUNK_SIZE, CHUNK_VOLUME};

pub struct ChunkMeshBuilder<'a> {
    p_chunk: &'a ChunkSection,
    p_meshes: &'a ChunkMeshCollection,
    p_block_data: Option<&'a BlockDataHolder>,
    p_active_mesh: Option<&'a ChunkMesh>
}

#[derive(Copy, Clone, Default)]
struct AdjacentBlockPositions {
    pub up: Vector3i,
    pub down: Vector3i,
    pub left: Vector3i,
    pub right: Vector3i,
    pub front: Vector3i,
    pub back: Vector3i
}

const FRONT_FACE: [GLfloat; 12] = [0., 0., 1., 1., 0., 1., 1., 1., 1., 0., 1., 1.];
const BACK_FACE: [GLfloat; 12] = [1., 0., 0., 0., 0., 0., 0., 1., 0., 1., 1., 0.];
const LEFT_FACE: [GLfloat; 12] = [0., 0., 0., 0., 0., 1., 0., 1., 1., 0., 1., 0.];
const RIGHT_FACE: [GLfloat; 12] = [1., 0., 1., 1., 0., 0., 1., 1., 0., 1., 1., 1.];
const TOP_FACE: [GLfloat; 12] = [0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 1., 0.];
const BOTTOM_FACE: [GLfloat; 12] = [0., 0., 0., 1., 0., 0., 1., 0., 1., 0., 0., 1.];

const X_FACE_1: [GLfloat; 12] = [0., 0., 0., 1., 0., 1., 1., 1., 1., 0., 1., 0.];
const X_FACE_2: [GLfloat; 12] = [0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1.];

const LIGHT_TOP: GLfloat = 1.0;
const LIGHT_X: GLfloat = 0.8;
const LIGHT_Z: GLfloat = 0.6;
const LIGHT_BOT: GLfloat = 0.4;

impl<'a> ChunkMeshBuilder<'a> {
    pub fn new(
        chunk: &ChunkSection,
        meshes: &ChunkMeshCollection
    ) -> Self {
        Self {
            p_chunk: chunk,
            p_meshes: meshes,
            p_block_data: None,
            p_active_mesh: None
        }
    }

    pub fn build_mesh(&mut self) {
        let mut directions = AdjacentBlockPositions::default();
        let mut block_iter = self.p_chunk.iter();
        for i in 0..CHUNK_VOLUME {
            let x = i % CHUNK_SIZE;
            let y = i / (CHUNK_SIZE * CHUNK_SIZE);
            let z = (i / CHUNK_SIZE) % CHUNK_SIZE;

            if !self.should_make_layer(y as _) {
                continue;
            }

            let block = block_iter.next().unwrap();

            let position = Vector3i::new(x as _, y as _, z as _);
            self.set_active_mesh(block);

            if block.id == BlockId::Air as _ {
                continue;
            }

            self.p_block_data = Some(block.get_data());
            let data = self.p_block_data.unwrap();

            if data.mesh_type == BlockMeshType::X {
                self.add_x_block_to_mesh(&data.tex_top_coord, &position);
            }

            directions.update(x as _, y as _, z as _);

            // Up/ Down
            if self.p_chunk.get_location().y != 0 || y != 0 {
                self.try_add_face_to_mesh(
                    BOTTOM_FACE,
                    &data.tex_bottom_coord,
                    &position,
                    &directions.down,
                    LIGHT_BOT
                );
            }
            self.try_add_face_to_mesh(
                TOP_FACE,
                &data.tex_top_coord,
                &position,
                &directions.up,
                LIGHT_TOP
            );

            // Left/ Right
            self.try_add_face_to_mesh(
                LEFT_FACE,
                &data.tex_side_coord,
                &position,
                &directions.left,
                LIGHT_X
            );
            self.try_add_face_to_mesh(
                RIGHT_FACE,
                &data.tex_side_coord,
                &position,
                &directions.right,
                LIGHT_X
            );

            // Front/ Back
            self.try_add_face_to_mesh(
                FRONT_FACE,
                &data.tex_side_coord,
                &position,
                &directions.front,
                LIGHT_Z
            );
            self.try_add_face_to_mesh(
                BACK_FACE,
                &data.tex_side_coord,
                &position,
                &directions.back,
                LIGHT_Z
            );
        }
    }

    fn set_active_mesh(&mut self, block: &ChunkBlock) {
        match block.get_data().shader_type {
            BlockShaderType::Chunk => {
                self.p_active_mesh = Some(&self.p_meshes.solid_mesh);
            }
            BlockShaderType::Liquid => {
                self.p_active_mesh = Some(&self.p_meshes.water_mesh);
            }
            BlockShaderType::Flora => {
                self.p_active_mesh = Some(&self.p_meshes.flora_mesh);
            }
        }
    }

    fn add_x_block_to_mesh(
        &mut self,
        texture_coords: &Vector2i,
        block_position: &Vector3i
    ) {
        let tex_coords = BlockDatabase::get().texture_atlas.get_texture(texture_coords);

        self.p_active_mesh.unwrap().add_face(X_FACE_1, tex_coords, &self.p_chunk.get_location(),
                                             block_position, LIGHT_X);
        self.p_active_mesh.unwrap().add_face(X_FACE_2, tex_coords, &self.p_chunk.get_location(),
                                             block_position, LIGHT_X);
    }

    fn try_add_face_to_mesh(
        &mut self,
        block_face: [GLfloat; 12],
        texture_coords: &Vector2i,
        block_position: &Vector3i,
        block_facing: &Vector3i,
        cardinal_light: GLfloat
    ) {
        if self.should_make_face(block_facing, self.p_block_data.unwrap()) {
            let tex_coords = BlockDatabase::get().texture_atlas.get_texture(texture_coords);

            self.p_active_mesh.unwrap().add_face(
                block_face,
                tex_coords,
                &self.p_chunk.get_location(),
                block_position,
                cardinal_light
            );
        }
    }

    fn should_make_face(
        &self,
        block_position: &Vector3i,
        block_data: &BlockDataHolder
    ) -> bool {
        let block = self.p_chunk.get_block(block_position.x, block_position.y, block_position.z);
        let data = block.get_data();

        if block.id == BlockId::Air as _ {
            true
        } else if !data.is_opaque && data.id != self.p_block_data.unwrap().id {
            true
        } else {
            false
        }
    }

    fn should_make_layer(&mut self, y: i32) -> bool {
        let adj_is_solid = |dx, dz| {
            let sect = self.p_chunk.get_adjacent(dx, dz);
            sect.get_layer(y).is_all_solid()
        };
        
        !self.p_chunk.get_layer(y).is_all_solid() ||
            self.p_chunk.get_layer(y + 1).is_all_solid() ||
            self.p_chunk.get_layer(y - 1).is_all_solid() ||
            
            !adj_is_solid(1, 0) ||
            !adj_is_solid(0, 1) ||
            !adj_is_solid(-1, 0) ||
            !adj_is_solid(0, -1)
    }
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