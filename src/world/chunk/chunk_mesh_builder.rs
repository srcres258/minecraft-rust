use std::cell::RefCell;
use std::rc::Rc;
use gl::types::GLfloat;
use sfml::system::{Vector2i, Vector3i};
use crate::world::block::block_data::{BlockData, BlockDataHolder, BlockMeshType, BlockShaderType};
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::IChunk;
use crate::world::chunk::chunk_mesh::ChunkMeshCollection;
use crate::world::chunk::chunk_section::ChunkSection;
use crate::world::world_constants::{CHUNK_SIZE, CHUNK_VOLUME};

pub struct ChunkMeshBuilder<'a> {
    p_chunk: &'a mut ChunkSection
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
        chunk: &'a mut ChunkSection
    ) -> Self {
        Self {
            p_chunk: chunk
        }
    }

    pub fn build_mesh(&mut self) {
        let mut directions = AdjacentBlockPositions::default();
        let mut block_iter = self.p_chunk.blocks.iter();
        for i in 0..CHUNK_VOLUME {
            let x = i % CHUNK_SIZE;
            let y = i / (CHUNK_SIZE * CHUNK_SIZE);
            let z = (i / CHUNK_SIZE) % CHUNK_SIZE;

            if !self.should_make_layer(y as _) {
                continue;
            }

            let block = block_iter.next().unwrap();

            let position = Vector3i::new(x as _, y as _, z as _);

            if block.id == BlockId::Air as _ {
                continue;
            }

            let p_block_data = block.get_data();
            let data = p_block_data.clone();

            if data.borrow().block_data().mesh_type == BlockMeshType::X {
                Self::add_x_block_to_mesh(&mut self.p_chunk.meshes, self.p_chunk.location, block, &data.borrow().block_data().tex_top_coord, &position);
            }

            directions.update(x as _, y as _, z as _);

            // let should_make_face = |p_block_data: Rc<RefCell<BlockData>>,
            //                         block_position: &Vector3i,
            //                         _block_data: &BlockDataHolder| {
            //     let block = self.p_chunk.get_block(block_position.x, block_position.y, block_position.z);
            //     let data = block.get_data();
            //
            //     if block.id == BlockId::Air as _ {
            //         true
            //     } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
            //         true
            //     } else {
            //         false
            //     }
            // };
            // Up/ Down
            if self.p_chunk.get_location().y != 0 || y != 0 {
                let smf = {
                    let block = self.p_chunk.get_block(directions.down.x, directions.down.y, directions.down.z);
                    let data = block.get_data();

                    if block.id == BlockId::Air as _ {
                        true
                    } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                        true
                    } else {
                        false
                    }
                };
                Self::try_add_face_to_mesh(
                    smf,
                    self.p_chunk.location,
                    &mut self.p_chunk.meshes,
                    block,
                    BOTTOM_FACE,
                    &data.borrow().block_data().tex_bottom_coord,
                    &position,
                    LIGHT_BOT
                );
            }
            let smf = {
                let block = self.p_chunk.get_block(directions.up.x, directions.up.y, directions.up.z);
                let data = block.get_data();

                if block.id == BlockId::Air as _ {
                    true
                } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                    true
                } else {
                    false
                }
            };
            Self::try_add_face_to_mesh(
                smf,
                self.p_chunk.location,
                &mut self.p_chunk.meshes,
                block,
                TOP_FACE,
                &data.borrow().block_data().tex_top_coord,
                &position,
                LIGHT_TOP
            );

            // Left/ Right
            let smf = {
                let block = self.p_chunk.get_block(directions.left.x, directions.left.y, directions.left.z);
                let data = block.get_data();

                if block.id == BlockId::Air as _ {
                    true
                } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                    true
                } else {
                    false
                }
            };
            Self::try_add_face_to_mesh(
                smf,
                self.p_chunk.location,
                &mut self.p_chunk.meshes,
                block,
                LEFT_FACE,
                &data.borrow().block_data().tex_side_coord,
                &position,
                LIGHT_X
            );
            let smf = {
                let block = self.p_chunk.get_block(directions.right.x, directions.right.y, directions.right.z);
                let data = block.get_data();

                if block.id == BlockId::Air as _ {
                    true
                } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                    true
                } else {
                    false
                }
            };
            Self::try_add_face_to_mesh(
                smf,
                self.p_chunk.location,
                &mut self.p_chunk.meshes,
                block,
                RIGHT_FACE,
                &data.borrow().block_data().tex_side_coord,
                &position,
                LIGHT_X
            );

            // Front/ Back
            let smf = {
                let block = self.p_chunk.get_block(directions.front.x, directions.front.y, directions.front.z);
                let data = block.get_data();

                if block.id == BlockId::Air as _ {
                    true
                } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                    true
                } else {
                    false
                }
            };
            Self::try_add_face_to_mesh(
                smf,
                self.p_chunk.location,
                &mut self.p_chunk.meshes,
                block,
                FRONT_FACE,
                &data.borrow().block_data().tex_side_coord,
                &position,
                LIGHT_Z
            );
            let smf = {
                let block = self.p_chunk.get_block(directions.back.x, directions.back.y, directions.back.z);
                let data = block.get_data();

                if block.id == BlockId::Air as _ {
                    true
                } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
                    true
                } else {
                    false
                }
            };
            Self::try_add_face_to_mesh(
                smf,
                self.p_chunk.location,
                &mut self.p_chunk.meshes,
                block,
                BACK_FACE,
                &data.borrow().block_data().tex_side_coord,
                &position,
                LIGHT_Z
            );
        }
    }

    fn add_x_block_to_mesh(
        meshes: &mut ChunkMeshCollection,
        location: Vector3i,
        block: &ChunkBlock,
        texture_coords: &Vector2i,
        block_position: &Vector3i
    ) {
        let tex_coords = BlockDatabase::get().texture_atlas.get_texture(texture_coords);

        match block.get_data().borrow().block_data().shader_type {
            BlockShaderType::Chunk => {
                meshes.solid_mesh.add_face(X_FACE_1, tex_coords, &location,
                                                        block_position, LIGHT_X);
                meshes.solid_mesh.add_face(X_FACE_2, tex_coords, &location,
                                                        block_position, LIGHT_X);
            }
            BlockShaderType::Liquid => {
                meshes.water_mesh.add_face(X_FACE_1, tex_coords, &location,
                                                        block_position, LIGHT_X);
                meshes.water_mesh.add_face(X_FACE_2, tex_coords, &location,
                                                        block_position, LIGHT_X);
            }
            BlockShaderType::Flora => {
                meshes.flora_mesh.add_face(X_FACE_1, tex_coords, &location,
                                                        block_position, LIGHT_X);
                meshes.flora_mesh.add_face(X_FACE_2, tex_coords, &location,
                                                        block_position, LIGHT_X);
            }
        }
    }

    fn try_add_face_to_mesh(
        should_make_face: bool,
        location: Vector3i,
        meshes: &mut ChunkMeshCollection,
        block: &ChunkBlock,
        block_face: [GLfloat; 12],
        texture_coords: &Vector2i,
        block_position: &Vector3i,
        cardinal_light: GLfloat
    ) {
        if should_make_face {
            let tex_coords = BlockDatabase::get().texture_atlas.get_texture(texture_coords);

            match block.get_data().borrow().block_data().shader_type {
                BlockShaderType::Chunk => {
                    meshes.solid_mesh.add_face(
                        block_face,
                        tex_coords,
                        &location,
                        block_position,
                        cardinal_light
                    );
                }
                BlockShaderType::Liquid => {
                    meshes.water_mesh.add_face(
                        block_face,
                        tex_coords,
                        &location,
                        block_position,
                        cardinal_light
                    );
                }
                BlockShaderType::Flora => {
                    meshes.flora_mesh.add_face(
                        block_face,
                        tex_coords,
                        &location,
                        block_position,
                        cardinal_light
                    );
                }
            }
        }
    }

    fn should_make_face(
        &self,
        p_block_data: Rc<RefCell<BlockData>>,
        block_position: &Vector3i,
        _block_data: &BlockDataHolder
    ) -> bool {
        let block = self.p_chunk.get_block(block_position.x, block_position.y, block_position.z);
        let data = block.get_data();

        if block.id == BlockId::Air as _ {
            true
        } else if !data.borrow().block_data().is_opaque && data.borrow().block_data().id != p_block_data.borrow().block_data().id {
            true
        } else {
            false
        }
    }

    fn should_make_layer(&self, y: i32) -> bool {
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