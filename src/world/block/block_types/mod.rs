use crate::world::block::block_data::BlockData;

pub struct BlockType {
    data: BlockData
}

pub struct DefaultBlock {}

impl BlockType {
    pub fn new(file_name: &str) -> Self {
        Self {
            data: BlockData::new(file_name)
        }
    }
    
    pub fn data(&self) -> &BlockData {
        &self.data
    }
}

impl DefaultBlock {
    pub fn new(file_name: &str) -> BlockType {
        BlockType::new(file_name)
    }
}