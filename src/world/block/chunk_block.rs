use crate::world::block::block_data::BlockDataHolder;
use crate::world::block::block_database::BlockDatabase;
use crate::world::block::block_id::{BlockId, BlockNumType};
use crate::world::block::block_types::BlockType;

#[derive(Copy, Clone)]
pub struct ChunkBlock {
    pub id: BlockNumType
}

impl ChunkBlock {
    pub fn from_block_t(id: BlockNumType) -> Self {
        Self { id }
    }
    pub fn from_block_id(id: BlockId) -> Self {
        Self {
            id: id as BlockNumType
        }
    }
    
    pub fn data(&self) -> &BlockDataHolder {
        BlockDatabase::get().data(BlockId::try_from(self.id).unwrap()).block_data()
    }
    pub fn type_(&self) -> &BlockType {
        BlockDatabase::get().block(BlockId::try_from(self.id).unwrap())
    }
}

impl Default for ChunkBlock {
    fn default() -> Self {
        Self::from_block_id(BlockId::Air)
    }
}

impl PartialEq<Self> for ChunkBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}