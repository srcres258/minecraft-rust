use crate::world::block::block_data::BlockDataHolder;
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
    
    pub fn get_data(&self) -> &BlockDataHolder {
        BlockDatabase::get()
            .get_data(BlockId::try_from(self.id as _).unwrap())
            .borrow()
            .block_data()
    }
}