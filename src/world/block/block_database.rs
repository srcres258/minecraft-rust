use crate::texture::texture_atlas::TextureAtlas;
use crate::world::block::block_id::BlockId;
use crate::world::block::block_types::{BlockType, DefaultBlock};
use lazy_static::lazy_static;
use crate::world::block::block_data::BlockData;

/// @brief Singleton class that determines status and ID of blocks as a whole.
pub struct BlockDatabase {
    pub texture_atlas: TextureAtlas,

    blocks: [Box<BlockType>; BlockId::NUM_TYPES]
}

lazy_static! {
    static ref DATABASE: BlockDatabase = BlockDatabase::new();
}

impl BlockDatabase {
    fn new() -> Self {
        Self {
            texture_atlas: TextureAtlas::new("DefaultPack").unwrap(),
            blocks: [
                Box::new(DefaultBlock::new("Air")),
                Box::new(DefaultBlock::new("Grass")),
                Box::new(DefaultBlock::new("Dirt")),
                Box::new(DefaultBlock::new("Stone")),
                Box::new(DefaultBlock::new("OakBark")),
                Box::new(DefaultBlock::new("OakLeaf")),
                Box::new(DefaultBlock::new("Sand")),
                Box::new(DefaultBlock::new("Water")),
                Box::new(DefaultBlock::new("Cactus")),
                Box::new(DefaultBlock::new("TallGrass")),
                Box::new(DefaultBlock::new("Rose")),
                Box::new(DefaultBlock::new("DeadShrub"))
            ]
        }
    }
    
    pub fn get() -> &'static Self {
        &DATABASE
    }

    pub fn block(&self, id: BlockId) -> &BlockType {
        &self.blocks[id as usize]
    }
    pub fn data(&self, id: BlockId) -> &BlockData {
        &self.blocks[id as usize].data()
    }
}