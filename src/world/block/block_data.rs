use std::fs::File;
use std::io::{BufRead, BufReader};
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

#[derive(Default)]
pub struct BlockData {
    data: BlockDataHolder
}

#[derive(Eq, PartialEq)]
enum DecodingState {
    Vacant,
    TexTop,
    TexSide,
    TexBottom,
    TexAll,
    Id,
    Opaque,
    Collidable,
    MeshType,
    ShaderType
}

impl TryFrom<i32> for BlockMeshType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == BlockMeshType::Cube as i32 => Ok(BlockMeshType::Cube),
            x if x == BlockMeshType::X as i32 => Ok(BlockMeshType::X),
            _ => Err(())
        }
    }
}

impl TryFrom<i32> for BlockShaderType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == BlockShaderType::Chunk as i32 => Ok(BlockShaderType::Chunk),
            x if x == BlockShaderType::Liquid as i32 => Ok(BlockShaderType::Liquid),
            x if x == BlockShaderType::Flora as i32 => Ok(BlockShaderType::Flora),
            _ => Err(())
        }
    }
}

impl Default for BlockDataHolder {
    fn default() -> Self {
        Self {
            id: BlockId::Air,
            tex_top_coord: Default::default(),
            tex_side_coord: Default::default(),
            tex_bottom_coord: Default::default(),
            mesh_type: BlockMeshType::Cube,
            shader_type: BlockShaderType::Chunk,
            is_opaque: false,
            is_collidable: false
        }
    }
}

impl BlockData {
    pub fn new(file_name: &str) -> Self {
        let mut result = Self::default();

        /* BlockData parses through text strings and applies valid attributes.

        Textures are applied first, then Block IDs, opacity data, collision data,
        mesh data, and shader data.

        Essentially, blocks being constructed by the renderer depend on this
        file data being correctly imported and read by the program.*/

        let in_file = BufReader::new(File::open(
            format!("Res/Blocks/{}.block", file_name))
            .expect(format!("Unable to open block file: {}!", file_name).as_str()));
        let mut state = DecodingState::Vacant;
        for line in in_file.lines() {
            let line = line.unwrap().trim();
            if state == DecodingState::Vacant {
                match line {
                    "TexTop" => state = DecodingState::TexTop,
                    "TexSide" => state = DecodingState::TexSide,
                    "TexBottom" => state = DecodingState::TexBottom,
                    "TexAll" => state = DecodingState::TexAll,
                    "Id" => state = DecodingState::Id,
                    "Opaque" => state = DecodingState::Opaque,
                    "Collidable" => state = DecodingState::Collidable,
                    "MeshType" => state = DecodingState::MeshType,
                    "ShaderType" => state = DecodingState::ShaderType,
                    _ => {}
                }
            } else {
                match state {
                    DecodingState::TexTop => {
                        let parts: Vec<_> = line.split(' ').collect();
                        let x = parts[0].parse::<i32>().unwrap();
                        let y = parts[1].parse::<i32>().unwrap();
                        result.data.tex_top_coord.x = x;
                        result.data.tex_top_coord.y = y;
                    }
                    DecodingState::TexSide => {
                        let parts: Vec<_> = line.split(' ').collect();
                        let x = parts[0].parse::<i32>().unwrap();
                        let y = parts[1].parse::<i32>().unwrap();
                        result.data.tex_side_coord.x = x;
                        result.data.tex_side_coord.y = y;
                    }
                    DecodingState::TexBottom => {
                        let parts: Vec<_> = line.split(' ').collect();
                        let x = parts[0].parse::<i32>().unwrap();
                        let y = parts[1].parse::<i32>().unwrap();
                        result.data.tex_bottom_coord.x = x;
                        result.data.tex_bottom_coord.y = y;
                    }
                    DecodingState::TexAll => {
                        let parts: Vec<_> = line.split(' ').collect();
                        let x = parts[0].parse::<i32>().unwrap();
                        let y = parts[1].parse::<i32>().unwrap();
                        result.data.tex_top_coord.x = x;
                        result.data.tex_top_coord.y = y;
                        result.data.tex_side_coord.x = x;
                        result.data.tex_side_coord.y = y;
                        result.data.tex_bottom_coord.x = x;
                        result.data.tex_bottom_coord.y = y;
                    }
                    DecodingState::Id => {
                        let id = line.parse::<i32>().unwrap();
                        result.data.id = BlockId::try_from(id).unwrap();
                    }
                    DecodingState::Opaque => {
                        result.data.is_opaque = if line == "1" { true } else { false }
                    }
                    DecodingState::Collidable => {
                        result.data.is_collidable = if line == "1" { true } else { false }
                    }
                    DecodingState::MeshType => {
                        let id = line.parse::<i32>().unwrap();
                        result.data.mesh_type = BlockMeshType::try_from(id).unwrap();
                    }
                    DecodingState::ShaderType => {
                        let id = line.parse::<i32>().unwrap();
                        result.data.shader_type = BlockShaderType::try_from(id).unwrap();
                    }
                    _ => {}
                }
                state = DecodingState::Vacant;
            }
        }

        result
    }

    pub fn block_data(&self) -> &BlockDataHolder {
        &self.data
    }
}