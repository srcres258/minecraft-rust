// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Mutex;
use lazy_static::lazy_static;
use sfml::system::Vector3i;
use crate::maths::general_maths::smooth_interpolation;
use crate::maths::noise_generator::{NoiseGenerator, NoiseParameters};
use crate::util::array2d::Array2D;
use crate::util::random::{Random, RandomSingleton};
use crate::world::block::block_id::BlockId;
use crate::world::block::chunk_block::ChunkBlock;
use crate::world::chunk::chunk::{Chunk, IChunk};
use crate::world::generation::biome::biome::Biome;
use crate::world::generation::biome::desert_biome::DesertBiome;
use crate::world::generation::biome::grassland_biome::GrasslandBiome;
use crate::world::generation::biome::light_forest::LightForest;
use crate::world::generation::biome::ocean_biome::OceanBiome;
use crate::world::generation::biome::temperate_forest_biome::TemperateForestBiome;
use crate::world::generation::terrain_generator::TerrainGenerator;
use crate::world::world_constants::{CHUNK_SIZE, WATER_LEVEL};

/// @brief Generates chunks based on perlin noise and recognizable MC parameters.
pub struct ClassicOverWorldGenerator {
    height_map: Array2D<i32>,
    biome_map: Array2D<i32>,

    random: Random,

    grass_biome: GrasslandBiome,
    temperate_forest: TemperateForestBiome,
    desert_biome: DesertBiome,
    ocean_biome: OceanBiome,
    light_forest: LightForest
}

lazy_static! {
    static ref SEED: i32 = RandomSingleton::get().int_in_range(424..=325322);

    static ref BIOME_NOISE_GEN: Mutex<NoiseGenerator> = Mutex::new(NoiseGenerator::new(SEED.clone() * 2));
}

static mut NOISE_GEN: bool = false;

impl ClassicOverWorldGenerator {
    pub fn new() -> Self {
        Self::set_up_noise();
        Self::default()
    }

    fn set_up_noise() {
        log::info!("Seed: {}", SEED.clone());
        unsafe {
            if !NOISE_GEN {
                log::info!("making noise");
                NOISE_GEN = true;

                let mut biome_params = NoiseParameters::default();
                biome_params.octaves = 5;
                biome_params.amplitude = 120;
                biome_params.smoothness = 1035;
                biome_params.height_offset = 0;
                biome_params.roughness = 0.75;

                BIOME_NOISE_GEN.lock().unwrap().set_parameters(biome_params);
            }
        }
    }

    fn set_blocks(&mut self, p_chunk: &mut Chunk, max_height: i32) {
        let mut trees: Vec<Vector3i> = Vec::new();
        let mut plants: Vec<Vector3i> = Vec::new();

        for y in 0..max_height + 1 {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let height = *self.height_map.get(x, z);
                    let biome = self.get_biome(x, z);

                    if y > height {
                        if y <= WATER_LEVEL as i32 {
                            p_chunk.set_block(x as _, y as _, z as _,
                                              ChunkBlock::new_with_block_id(BlockId::Water));
                        }
                        continue;
                    } else if y == height {
                        if y >= WATER_LEVEL as i32 {
                            if y < (WATER_LEVEL + 4) as i32 {
                                p_chunk.set_block(x as _, y as _, z as _, biome.get_beach_block(&self.random));
                                continue;
                            }

                            if self.random.int_in_range(0..=biome.get_tree_frequency()) == 5 {
                                trees.push(Vector3i::new(x as _, y + 1, z as _));
                            }
                            if self.random.int_in_range(0..=biome.get_tree_frequency()) == 5 {
                                plants.push(Vector3i::new(x as _, y + 1, z as _));
                            }
                            p_chunk.set_block(x as _, y, z as _, self.get_biome(x, z).get_top_block(&self.random));
                        } else {
                            p_chunk.set_block(x as _, y, z as _, biome.get_under_water_block(&self.random));
                        }
                    } else if y > height - 3 {
                        p_chunk.set_block(x as _, y, z as _, ChunkBlock::new_with_block_id(BlockId::Dirt));
                    } else {
                        p_chunk.set_block(x as _, y, z as _, ChunkBlock::new_with_block_id(BlockId::Stone));
                    }
                }
            }
        }

        for plant in plants.iter() {
            let x = plant.x;
            let z = plant.z;

            let block = self.get_biome(x as _, z as _).get_plant(&self.random);
            p_chunk.set_block(x, plant.y, z, block);
        }

        for tree in trees.iter() {
            let x = tree.x;
            let z = tree.z;

            self.get_biome(x as _, z as _).make_tree(&self.random, p_chunk, x, tree.y, z);
        }
    }

    fn get_height_in(
        &mut self,
        p_chunk: &mut Chunk,
        x_min: i32,
        z_min: i32,
        x_max: i32,
        z_max: i32
    ) {
        let get_height_at = |x: i32, z: i32| {
            let biome = self.get_biome(x as _, z as _);

            biome.get_height(x, z, p_chunk.get_location().x, p_chunk.get_location().y)
        };

        let bottom_left = get_height_at(x_min, z_min) as f32;
        let bottom_right = get_height_at(x_max, z_min) as f32;
        let top_left = get_height_at(x_min, z_max) as f32;
        let top_right = get_height_at(x_max, z_max) as f32;

        for x in x_min..x_max {
            for z in z_min..z_max {
                if x == CHUNK_SIZE as _ {
                    continue;
                }
                if z == CHUNK_SIZE as _ {
                    continue;
                }

                let h = smooth_interpolation(
                    bottom_left, top_left, bottom_right, top_right,
                    x_min as _, x_max as _,
                    z_min as _, z_max as _,
                    x as _, z as _
                );

                *self.height_map.get_mut(x as _, z as _) = h as i32;
            }
        }
    }

    fn get_height_map(&mut self, p_chunk: &mut Chunk) {
        const HALF_CHUNK: i32 = CHUNK_SIZE as i32 / 2;
        const CHUNK: i32 = CHUNK_SIZE as i32;

        self.get_height_in(p_chunk, 0, 0, HALF_CHUNK, HALF_CHUNK);
        self.get_height_in(p_chunk, HALF_CHUNK, 0, CHUNK, HALF_CHUNK);
        self.get_height_in(p_chunk, 0, HALF_CHUNK, HALF_CHUNK, CHUNK);
        self.get_height_in(p_chunk, HALF_CHUNK, HALF_CHUNK, CHUNK, CHUNK);
    }

    fn get_biome_map(&mut self, p_chunk: &mut Chunk) {
        let location = p_chunk.get_location();

        for x in 0..=CHUNK_SIZE {
            for z in 0..=CHUNK_SIZE {
                let h = BIOME_NOISE_GEN.lock().unwrap().get_height(
                    x as _, z as _, location.x + 10, location.y + 10);
                *self.biome_map.get_mut(x, z) = h as i32;
            }
        }
    }

    fn get_biome(&self, x: usize, z: usize) -> &dyn Biome {
        let biome_value = *self.biome_map.get(x, z);

        if biome_value > 160 {
            &self.ocean_biome
        } else if biome_value > 150 {
            &self.grass_biome
        } else if biome_value > 130 {
            &self.light_forest
        } else if biome_value > 120 {
            &self.temperate_forest
        } else if biome_value > 110 {
            &self.light_forest
        } else if biome_value > 100 {
            &self.grass_biome
        } else {
            &self.desert_biome
        }
    }
}

impl Default for ClassicOverWorldGenerator {
    fn default() -> Self {
        Self {
            height_map: Array2D::new(CHUNK_SIZE),
            biome_map: Array2D::new(CHUNK_SIZE + 1),
            random: Default::default(),
            grass_biome: GrasslandBiome::new(SEED.clone()),
            temperate_forest: TemperateForestBiome::new(SEED.clone()),
            desert_biome: DesertBiome::new(SEED.clone()),
            ocean_biome: OceanBiome::new(SEED.clone()),
            light_forest: LightForest::new(SEED.clone())
        }
    }
}

impl TerrainGenerator for ClassicOverWorldGenerator {
    
    fn generate_terrain_for(&mut self, chunk: &mut Chunk) {
        let location = chunk.get_location();
        self.random.set_seed(((location.x ^ location.y) << 2) as _);

        self.get_biome_map(chunk);
        self.get_height_map(chunk);

        let mut max_height = *self.height_map.get_max_value();

        max_height = max_height.max(WATER_LEVEL as _);
        self.set_blocks(chunk, max_height);
    }

    fn get_minimum_spawn_height(&self) -> i32 {
        WATER_LEVEL as _
    }
}