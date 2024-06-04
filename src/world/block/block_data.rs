use sfml::system::Vector2i;
use crate::world::block::block_id::BlockId;

/// @brief Allocates meshes to cubes and non-cube entities.
pub enum BlockMeshType {
    Cube = 0,
    X = 1
}

/// @brief Allocates shader behavior to groups of blocks.
pub enum BlockShaderType {
    Chunk = 0,
    Liquid = 1,
    Flora = 2
}

/// @brief Struct designed to hold geometric and tangibility data for each individual block.
pub struct BlockDataHolder {
    pub id: BlockId,
    pub tex_top_coord: Vector2i,
    pub tex_side_coord: Vector2i,
    pub tex_bottom_coord: Vector2i,

    pub mesh_type: BlockMeshType,
    pub shader_type: BlockShaderType,

    pub is_opaque: bool,
    pub is_collidable: bool
}

pub struct BlockData {
    data: BlockDataHolder
}

impl BlockData {
    //todo
}