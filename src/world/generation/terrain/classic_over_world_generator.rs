use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;
use sfml::system::Vector3i;
use crate::maths::noise_generator::{NoiseGenerator, NoiseParameters};
use crate::maths::smooth_interpolation;
use crate::util::array_2d::Array2D;
use crate::util::mem::{uw_ref, uw_ref_mut, UWRefMut};
use crate::util::random::{Random, RandomSigleton};
use crate::util::singleton::Singleton;
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};
use crate::world::constants::{CHUNK_SIZE, WATER_LEVEL};
use crate::world::generation::biome::biome::Biome;
use crate::world::generation::biome::desert_biome::DesertBiome;
use crate::world::generation::biome::grassland_biome::GrasslandBiome;
use crate::world::generation::biome::light_forest::LightForest;
use crate::world::generation::biome::ocean_biome::OceanBiome;
use crate::world::generation::biome::temperate_forest_biome::TemperateForestBiome;
use crate::world::generation::terrain_generator::TerrainGenerator;

/// @brief Generates chunks based on perlin noise and recognizable MC parameters.
pub struct ClassicOverWorldGenerator {
    height_map: Array2D<i32, CHUNK_SIZE>,
    biome_map: Array2D<i32, { CHUNK_SIZE + 1 }>,

    random: Random,

    biome_noise_gen: NoiseGenerator,

    grassland: GrasslandBiome,
    temperate_forest: TemperateForestBiome,
    desert: DesertBiome,
    ocean: OceanBiome,
    light_forest: LightForest,

    chunk: Option<UWRefMut<Chunk>>,

    noise_gen: bool
}

static SEED: LazyLock<i32> = LazyLock::new(|| RandomSigleton::get().i32_in_range(424, 325322));

impl ClassicOverWorldGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    fn set_blocks(&mut self, max_height: i32) {
        let mut trees = Vec::new();
        let mut plants = Vec::new();
        
        for y in 0 .. max_height + 1 {
            for x in 0 .. CHUNK_SIZE as i32 {
                for z in 0 .. CHUNK_SIZE as i32 {
                    let height = *self.height_map.get(x as _, z as _);
                    let self1 = uw_ref(self);
                    let biome = self1.biome(x, z);
                    
                    if y > height {
                        if y <= WATER_LEVEL as _ {
                            self.chunk.as_mut().unwrap().set_block(
                                x, y, z,
                                ChunkBlock::from_block_id(BlockId::Water)
                            );
                        }
                        continue;
                    } else if y == height {
                        if y >= WATER_LEVEL as _ {
                            if y < WATER_LEVEL as i32 + 4 {
                                self.chunk.as_mut().unwrap().set_block(
                                    x, y, z,
                                    biome.beach_block(&mut self.random)
                                );
                                continue;
                            }
                            
                            if self.random.i32_in_range(0, biome.tree_frequency()) == 5 {
                                trees.push(Vector3i::new(x, y + 1, z));
                            }
                            if self.random.i32_in_range(0, biome.plant_frequency()) == 5 {
                                plants.push(Vector3i::new(x, y + 1, z));
                            }
                            self.chunk.as_mut().unwrap().set_block(
                                x, y, z,
                                biome.top_block(&mut self.random)
                            );
                        } else {
                            self.chunk.as_mut().unwrap().set_block(
                                x, y, z,
                                biome.under_water_block(&mut self.random)
                            );
                        }
                    }
                }
            }
        }
        
        for plant in plants.iter() {
            let x = plant.x;
            let z = plant.z;
            
            let block = uw_ref(self).biome(x, z).plant(&mut self.random);
            self.chunk.as_mut().unwrap().set_block(x, plant.y, z, block);
        }
        
        for tree in trees.iter() {
            let x = tree.x;
            let y = tree.y;
            let z = tree.z;
            
            uw_ref(self).biome(x, z).make_tree(
                &mut self.random, self.chunk.as_mut().unwrap().deref_mut(),
                x, y, z
            );
        }
    }

    fn setup_noise(&mut self) {
        let seed = *SEED;
        log::info!("Seed: {}", seed);
        if !self.noise_gen {
            log::info!("making noise");
            self.noise_gen = true;

            let mut params = NoiseParameters::default();
            params.octaves = 5;
            params.amplitude = 120;
            params.smoothness = 1035;
            params.height_offset = 0;
            params.roughness = 0.75;

            self.biome_noise_gen.set_parameters(params);
        }
    }

    fn get_height_in(&mut self, x_min: i32, z_min: i32, x_max: i32, z_max: i32) {
        let height_at = |x, z| {
            let biome = self.biome(x, z);

            biome.height(x, z, self.chunk.as_ref().unwrap().location().x,
                         self.chunk.as_ref().unwrap().location().y)
        };

        let bottom_left = height_at(x_min, z_min);
        let bottom_right = height_at(x_max, z_min);
        let top_left = height_at(x_min, z_max);
        let top_right = height_at(x_max, z_max);

        for x in x_min .. x_max {
            for z in z_min .. z_max {
                if x == CHUNK_SIZE as _ {
                    continue;
                }
                if z == CHUNK_SIZE as _ {
                    continue;
                }

                let h = smooth_interpolation(
                    bottom_left as _, top_left as _,
                    bottom_right as _, top_right as _,
                    x_min as _, x_max as _,
                    z_min as _, z_max as _,
                    x as _, z as _
                );

                *self.height_map.get_mut(x as _, z as _) = h as i32;
            }
        }
    }
    fn get_height_map(&mut self) {
        const HALF_CHUNK: i32 = CHUNK_SIZE as i32 / 2;
        const CHUNK: i32 = CHUNK_SIZE as i32;

        self.get_height_in(0, 0, HALF_CHUNK, HALF_CHUNK);
        self.get_height_in(0, 0, CHUNK, HALF_CHUNK);
        self.get_height_in(0, HALF_CHUNK, HALF_CHUNK, CHUNK);
        self.get_height_in(HALF_CHUNK, HALF_CHUNK, CHUNK, CHUNK);
    }
    fn get_biome_map(&mut self) {
        let location = self.chunk.as_ref().unwrap().location();
        
        for x in 0 .. CHUNK_SIZE as i32 + 1 {
            for z in 0 .. CHUNK_SIZE as i32 + 1 {
                let h = self.biome_noise_gen.height(x, z, location.x + 10, location.y + 10);
                *self.biome_map.get_mut(x as _, z as _) = h as i32;
            }
        }
    }

    fn biome(&self, x: i32, z: i32) -> &dyn Biome {
        let biome_value = *self.biome_map.get(x as _, z as _);
        
        if biome_value > 160 {
            &self.ocean
        } else if biome_value > 150 {
            &self.grassland
        } else if biome_value > 130 {
            &self.light_forest
        } else if biome_value > 120 {
            &self.temperate_forest
        } else if biome_value > 110 {
            &self.light_forest
        } else if biome_value > 100 {
            &self.grassland
        } else {
            &self.desert
        }
    }
}

impl TerrainGenerator for ClassicOverWorldGenerator {
    fn generate_terrain_for(&mut self, chunk: &mut Chunk) {
        self.chunk = Some(uw_ref_mut(chunk));

        let location = chunk.location();
        self.random.set_seed(((location.x ^ location.y) << 2) as u64);

        self.get_biome_map();
        self.get_height_map();

        let mut max_height = *self.height_map.max_value();

        max_height = max_height.max(WATER_LEVEL as _);
        self.set_blocks(max_height);
    }
    fn minimum_spawn_height(&self) -> i32 {
        WATER_LEVEL as _
    }
}

impl Default for ClassicOverWorldGenerator {
    fn default() -> Self {
        let seed = *SEED;
        Self {
            height_map: Array2D::default(),
            biome_map: Array2D::default(),
            random: Random::default(),
            biome_noise_gen: NoiseGenerator::default(),
            grassland: GrasslandBiome::new(seed),
            temperate_forest: TemperateForestBiome::new(seed),
            desert: DesertBiome::new(seed),
            ocean: OceanBiome::new(seed),
            light_forest: LightForest::new(seed),
            chunk: None,
            noise_gen: false
        }
    }
}