use std::sync::{Arc, RwLock};
use crate::world::block::block_data::BlockData;
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::{BlockId, BlockType};

#[derive(Copy, Clone, Default)]
pub struct ChunkBlock {
    pub id: BlockType
}

impl ChunkBlock {
    pub fn new_with_block_type(id: BlockType) -> Self {
        Self { id }
    }

    pub fn new_with_block_id(id: BlockId) -> Self {
        Self { id: id as BlockType }
    }
    
    pub fn get_data(&self) -> Arc<RwLock<BlockData>> {
        BlockDatabase::get()
            .get_data(BlockId::try_from(self.id as i32).unwrap())
    }
}