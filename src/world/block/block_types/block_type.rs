use std::cell::RefCell;
use std::rc::Rc;
use crate::world::block::block_data::BlockData;

pub trait BlockType {
    fn data(&self) -> Rc<RefCell<BlockData>>;
}

pub struct DefaultBlock {
    data: Rc<RefCell<BlockData>>
}

impl DefaultBlock {
    pub fn new(file_name: &str) -> Self {
        Self {
            data: Rc::new(RefCell::new(BlockData::new(file_name)))
        }
    }
}

impl BlockType for DefaultBlock {
    fn data(&self) -> Rc<RefCell<BlockData>> {
        Rc::clone(&self.data)
    }
}