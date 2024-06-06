use rand::prelude::StdRng;
use rand::Rng;
use crate::util::random::Random;
use crate::world::block::block_id::BlockId;
use crate::world::chunk::chunk::Chunk;
use crate::world::generation::structure::structure_builder::StructureBuilder;

const CACTUS: BlockId = BlockId::Cactus;

fn make_cactus_1(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let mut builder = StructureBuilder::default();
    builder.make_column(x, z, y, rand.int_in_range(4..=7), CACTUS);
    builder.build(chunk);
}

fn make_cactus_2(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let mut builder = StructureBuilder::default();
    let height: i32 = rand.int_in_range(6..=8);
    builder.make_column(x, z, y, height, CACTUS);

    let stem = height / 2;

    builder.make_row_x(x - 2, x + 2, stem + y, z, CACTUS);
    builder.add_block(x - 2, stem + y + 1, z, CACTUS);
    builder.add_block(x - 2, stem + y + 2, z, CACTUS);
    builder.add_block(x + 2, stem + y + 1, z, CACTUS);

    builder.build(chunk);
}

fn make_cactus_3(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let mut builder = StructureBuilder::default();
    let height: i32 = rand.int_in_range(6..=8);
    builder.make_column(x, z, y, height, CACTUS);

    let stem = height / 2;

    builder.make_row_x(x - 2, x + 2, stem + y, z, CACTUS);
    builder.add_block(x, stem + y + 1, z - 2, CACTUS);
    builder.add_block(x, stem + y + 2, z - 2, CACTUS);
    builder.add_block(x, stem + y + 1, z + 2, CACTUS);

    builder.build(chunk);
}

pub fn make_oak_tree(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let mut builder = StructureBuilder::default();

    let h: i32 = rand.int_in_range(4..=7);
    let leaf_size = 2;

    let new_y = h + y;
    builder.fill(new_y, x - leaf_size, x + leaf_size,
                 z - leaf_size, z + leaf_size,
                 BlockId::OakLeaf);
    builder.fill(new_y - 1, x - leaf_size, x + leaf_size,
                 z - leaf_size, z + leaf_size,
                 BlockId::OakLeaf);

    for z_leaf in -leaf_size + 1 ..= leaf_size - 1 {
        builder.add_block(x, new_y + 1, z + z_leaf, BlockId::OakLeaf);
    }

    for x_leaf in -leaf_size + 1 ..= leaf_size - 1 {
        builder.add_block(x + x_leaf, new_y + 1, z, BlockId::OakLeaf);
    }

    builder.make_column(x, z, y, h, BlockId::OakBark);
    builder.build(chunk);
}

pub fn make_palm_tree(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let mut builder = StructureBuilder::default();

    let height: i32 = rand.int_in_range(7..=9);
    let diameter: i32 = rand.int_in_range(4..=6);

    for x_leaf in -diameter .. diameter {
        builder.add_block(x_leaf + x, y + height, z, BlockId::OakLeaf);
    }
    for z_leaf in -diameter .. diameter {
        builder.add_block(x, y + height, z_leaf + z, BlockId::OakLeaf);
    }

    builder.add_block(x, y + height - 1, z + diameter, BlockId::OakLeaf);
    builder.add_block(x, y + height - 1, z - diameter, BlockId::OakLeaf);
    builder.add_block(x + diameter, y + height - 1, z, BlockId::OakLeaf);
    builder.add_block(x - diameter, y + height - 1, z, BlockId::OakLeaf);
    builder.add_block(x, y + height - 1, z, BlockId::OakLeaf);

    builder.make_column(x, z, y, height, BlockId::OakBark);
    builder.build(chunk);
}

pub fn make_cactus(chunk: &mut Chunk, rand: &Random, x: i32, y: i32, z: i32) {
    let cac: i32 = rand.int_in_range(0..=2);

    match cac {
        0 => {
            make_cactus_1(chunk, rand, x, y, z);
        }
        1 => {
            make_cactus_2(chunk, rand, x, y, z);
        }
        2 => {
            make_cactus_3(chunk, rand, x, y, z);
        }
        _ => {}
    }
}