use std::cell::RefCell;
use std::rc::Rc;
use lazy_static::lazy_static;
use crate::texture::texture_atlas::TextureAtlas;
use crate::world::block::block_data::BlockData;
use crate::world::block::block_id::BlockId;
use crate::world::block::block_types::block_type::{BlockType, DefaultBlock};

lazy_static! {
    static ref INSTANCE: BlockDatabase = BlockDatabase::new();
}

/// @brief Singleton class that determines status and ID of blocks as a whole.
pub struct BlockDatabase {
    pub texture_atlas: TextureAtlas,
    blocks: [Box<dyn BlockType>; BlockId::NUM_TYPES]
}

impl BlockDatabase {
    fn new() -> Self {
        let texture_atlas = TextureAtlas::new("DefaultPack");
        let blocks = [
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
        ];
        Self { texture_atlas, blocks }
    }

    pub fn get() -> &'static Self {
        &INSTANCE
    }

    pub fn get_block(&self, id: BlockId) -> &dyn BlockType {
        self.blocks[id as _]
    }

    pub fn get_data(&self, id: BlockId) -> Rc<RefCell<BlockData>> {
        self.blocks[id as _].data()
    }
}