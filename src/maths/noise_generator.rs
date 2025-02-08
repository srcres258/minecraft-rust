use std::f64::consts::PI;
use std::num::Wrapping;
use crate::world::constants::{CHUNK_SIZE, WATER_LEVEL};

#[derive(Copy, Clone, Default, Debug)]
pub struct NoiseParameters {
    pub octaves: i32,
    pub amplitude: i32,
    pub smoothness: i32,
    pub height_offset: i32,

    pub roughness: f64
}

/// @brief Perlin noise generator used in construction of chunks and chunk blocks.
#[derive(Clone, Default, Debug)]
pub struct NoiseGenerator {
    noise_parameters: NoiseParameters,
    
    seed: i32
}

impl NoiseGenerator {
    pub fn new(seed: i32) -> Self {
        Self {
            noise_parameters: NoiseParameters {
                octaves: 7,
                amplitude: 70,
                smoothness: 235,
                height_offset: -5,
                roughness: 0.53
            },
            seed
        }
    }

    /// @brief Gets the height of the chunk for the sake of Noise Generation.
    /// @param x 
    /// @param z 
    /// @param chunkX 
    /// @param chunkZ 
    /// @return val
    pub fn height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> f64 {
        let new_x = x + chunk_x * CHUNK_SIZE as i32;
        let new_z = z + chunk_z * CHUNK_SIZE as i32;
        
        if new_x < 0 || new_z < 0 {
            return (WATER_LEVEL - 1) as f64;
        }
        
        let mut total_value = 0.0;
        
        for a in 0 .. self.noise_parameters.octaves - 1 { // This loops through the octaves.
            let frequency = 2f64.powi(a); // This increases the frequency with every loop of the octave.
            let amplitude = self.noise_parameters.roughness.powi(a); // This decreases the amplitude with every loop of the octave.
            total_value += self.noise(
                new_x as f64 * frequency / self.noise_parameters.smoothness as f64,
                new_z as f64 * frequency / self.noise_parameters.smoothness as f64
            ) * amplitude;
        }
        
        let val = (total_value / 2.1 + 1.2) * self.noise_parameters.amplitude as f64 +
            self.noise_parameters.height_offset as f64;
        
        if val > 0. { val } else { 1. } // Compare if value is greater than 0
    }
    
    pub fn set_parameters(&mut self, params: NoiseParameters) {
        self.noise_parameters = params;
    }

    /// @brief Gets Noise through n which acts as a seed number.
    /// @param n 
    /// @return 
    fn noise_n(&self, n: i32) -> f64 {
        let mut n = Wrapping(n);
        n += self.seed;
        n = n << 13 ^ n;
        let new_n = (n * (n * n * Wrapping(60493) + Wrapping(19990303)) + Wrapping(1376312589)) & Wrapping(0x7FFFFFFF);
        
        1.0 - new_n.0 as f64 / 1073741824.0
    }
    /// @brief Overload of getNoise that takes doubles instead of int n.
    /// @param x 
    /// @param z 
    /// @return 
    fn noise_xz(&self, x: f64, z: f64) -> f64 {
        self.noise_n((x + z * 57.0) as i32)
    }
    
    fn lerp(&self, a: f64, b: f64, c: f64) -> f64 {
        let mu2 = (1. - (c * PI).cos()) / 2.;
        a * (1. - mu2) + b * mu2
    }
    
    fn noise(&self, x: f64, z: f64) -> f64 {
        let floor_x = x.floor();
        let floor_z = z.floor();
        
        let s = self.noise_xz(floor_x, floor_z);
        let t = self.noise_xz(floor_x + 1., floor_z);
        let u = self.noise_xz(floor_x, floor_z + 1.); // Get the surrounding values to calculate the transition.
        let v = self.noise_xz(floor_x + 1., floor_z + 1.);
        
        let rec1 = self.lerp(s, t, x - floor_x); // Interpolate between the values.
        // Here we use x-floorX, to get 1st dimension. Don't mind
        // the x-floorX thingie, it's part of the cosine formula.
        let rec2 = self.lerp(u, v, x - floor_x);
        // Here we use y-floorZ, to get the 2nd dimension.
        let rec3 = self.lerp(rec1, rec2, z - floor_z);
        rec3
    }
}