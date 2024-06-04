pub trait BlockType<'a> {
    fn data(&self) -> &'a BlockData;
}

pub struct DefaultBlock {
    data: BlockData
}