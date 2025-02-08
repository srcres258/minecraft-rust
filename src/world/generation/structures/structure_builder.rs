use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};

#[derive(Copy, Clone, Default)]
struct Block {
    pub id: BlockId,
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Clone, Default)]
pub struct StructureBuilder {
    blocks: Vec<Block>
}

impl StructureBuilder {
    pub fn build(&mut self, chunk: &mut Chunk) {
        self.blocks.iter_mut().for_each(|block|
            chunk.set_block(block.x, block.y, block.z, ChunkBlock::from_block_id(block.id)));
    }

    pub fn make_column(&mut self, x: i32, z: i32, y_start: i32, height: i32, block: BlockId) {
        for y in y_start .. y_start + height {
            self.add_block(x, y, z, block);
        }
    }
    pub fn make_row_x(&mut self, x_start: i32, x_end: i32, y: i32, z: i32, block: BlockId) {
        for x in x_start ..= x_end {
            self.add_block(x, y, z, block);
        }
    }
    pub fn make_row_z(&mut self, z_start: i32, z_end: i32, x: i32, y: i32, block: BlockId) {
        for z in z_start ..= z_end {
            self.add_block(x, y, z, block);
        }
    }

    pub fn fill(&mut self, y: i32, x_start: i32, x_end: i32, z_start: i32, z_end: i32, block: BlockId) {
        for x in x_start .. x_end {
            for z in z_start .. z_end {
                self.add_block(x, y, z, block);
            }
        }
    }

    pub fn add_block(&mut self, x: i32, y: i32, z: i32, block: BlockId) {
        let mut result = Block::default();
        result.id = block;
        result.x = x;
        result.y = y;
        result.z = z;
        self.blocks.push(result);
    }
}