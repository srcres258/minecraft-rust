use std::sync::{Arc, RwLock};
use crate::world::block::block_data::BlockData;

pub trait BlockType {
    fn data(&self) -> Arc<RwLock<BlockData>>;
}

pub struct DefaultBlock {
    data: Arc<RwLock<BlockData>>
}

impl DefaultBlock {
    pub fn new(file_name: &str) -> Self {
        Self {
            data: Arc::new(RwLock::new(BlockData::new(file_name)))
        }
    }
}

impl BlockType for DefaultBlock {
    fn data(&self) -> Arc<RwLock<BlockData>> {
        Arc::clone(&self.data)
    }
}