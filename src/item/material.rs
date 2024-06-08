use lazy_static::lazy_static;
use crate::world::block::block_id::BlockId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ID {
    Nothing,
    Grass,
    Dirt,
    Stone,
    OakBark,
    OakLeaf,
    Sand,
    Cactus,
    Rose,
    TallGrass,
    DeadShrub
}

/// @brief Determines case-by-case properties and behaviors of known block types.
pub struct Material {
    pub id: ID,
    pub max_stack_size: i32,
    pub is_block: bool,
    pub name: String
}

impl Material {
    pub fn new(
        id: ID,
        max_stack_size: i32,
        is_block: bool,
        name: &str
    ) -> Self {
        Self {
            id,
            max_stack_size,
            is_block,
            name: String::from(name)
        }
    }

    pub fn to_block_id(&self) -> BlockId {
        match self.id {
            ID::Nothing => BlockId::Air,
            ID::Grass => BlockId::Grass,
            ID::Dirt => BlockId::Dirt,
            ID::Stone => BlockId::Stone,
            ID::OakBark => BlockId::OakBark,
            ID::OakLeaf => BlockId::OakLeaf,
            ID::Sand => BlockId::Sand,
            ID::Cactus => BlockId::Cactus,
            ID::TallGrass => BlockId::TallGrass,
            ID::Rose => BlockId::Rose,
            ID::DeadShrub => BlockId::DeadShrub
        }
    }

    pub fn from_block_id(id: BlockId) -> &'static Material {
        match id {
            BlockId::Grass => &GRASS_BLOCK,
            BlockId::Dirt => &DIRT_BLOCK,
            BlockId::Stone => &STONE_BLOCK,
            BlockId::OakBark => &OAK_BARK_BLOCK,
            BlockId::OakLeaf => &OAK_LEAF_BLOCK,
            BlockId::Sand => &SAND_BLOCK,
            BlockId::Cactus => &CACTUS_BLOCK,
            BlockId::Rose => &ROSE,
            BlockId::TallGrass => &TALL_GRASS,
            BlockId::DeadShrub => &DEAD_SHRUB,
            _ => &NOTHING
        }
    }
}

lazy_static! {
    pub static ref NOTHING: Material = Material::new(ID::Nothing, 0, false, "None");
    pub static ref GRASS_BLOCK: Material = Material::new(ID::Grass, 99, true, "Grass Block");
    pub static ref DIRT_BLOCK: Material = Material::new(ID::Dirt, 99, true, "Dirt Block");
    pub static ref STONE_BLOCK: Material = Material::new(ID::Stone, 99, true, "Stone Block");
    pub static ref OAK_BARK_BLOCK: Material = Material::new(ID::OakBark, 99, true, "Oak Bark Block");
    pub static ref OAK_LEAF_BLOCK: Material = Material::new(ID::OakLeaf, 99, true, "Oak Leaf Block");
    pub static ref SAND_BLOCK: Material = Material::new(ID::Sand, 99, true, "Sand Block");
    pub static ref CACTUS_BLOCK: Material = Material::new(ID::Cactus, 99, true, "Cactus Block");
    
    pub static ref ROSE: Material = Material::new(ID::Rose, 99, true, "Rose");
    pub static ref TALL_GRASS: Material = Material::new(ID::TallGrass, 99, true, "Tall Grass");
    pub static ref DEAD_SHRUB: Material = Material::new(ID::DeadShrub, 99, true, "Dead Shrub");
}