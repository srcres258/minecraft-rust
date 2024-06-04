use crate::world::world_constants::{CHUNK_SIZE, WATER_LEVEL};

#[derive(Copy, Clone, Default)]
pub struct NoiseParameters {
    octaves: i32,
    amplitude: i32,
    smoothness: i32,
    height_offset: i32,

    roughness: f64
}

/// @brief Perlin noise generator used in construction of chunks and chunk blocks.
pub struct NoiseGenerator {
    noise_parameters: NoiseParameters,
    seed: i32
}

impl NoiseParameters {
    pub fn new(
        octaves: i32,
        amplitude: i32,
        smoothness: i32,
        height_offset: i32,
        roughness: f64
    ) -> Self {
        Self { octaves, amplitude, smoothness, height_offset, roughness }
    }
}

impl NoiseGenerator {
    pub fn new(seed: i32) -> Self {
        let mut result = Self {
            noise_parameters: Default::default(),
            seed
        };

        result.noise_parameters.octaves = 7;
        result.noise_parameters.amplitude = 70;
        result.noise_parameters.smoothness = 235;
        result.noise_parameters.height_offset = -5;
        result.noise_parameters.roughness = 0.53;

        result
    }

    /// @brief Gets the height of the chunk for the sake of Noise Generation.
    /// @param x
    /// @param z
    /// @param chunkX
    /// @param chunkZ
    /// @return val
    pub fn get_height(&self, x: i32, z: i32, chunk_x: i32, chunk_z: i32) -> f64 {
        let new_x = x + chunk_x * CHUNK_SIZE;
        let new_z = z + chunk_z * CHUNK_SIZE;

        if new_x < 0 || new_z < 0 {
            return WATER_LEVEL as f64 - 1.0;
        }

        let mut total_value = 0f64;

        for a in 0..self.noise_parameters.octaves - 1 { // This loops through the octaves.
            let frequency = 2f64.powi(a); // This increases the frequency with every loop of the octave.
            let amplitude = self.noise_parameters.roughness.powi(a); // This decreases the amplitude with every loop of the octave.
            total_value += self.noise(
                new_x as f64 * frequency / self.noise_parameters.smoothness as f64,
                new_z as f64 * frequency / self.noise_parameters.smoothness as f64
            ) * amplitude;
        }

        let val = (total_value / 2.1 + 1.2) * self.noise_parameters.amplitude as f64
            + self.noise_parameters.height_offset as f64;

        if val > 0.0 { val } else { 1.0 } // Compare if value is greater than 0
    }

    /// @brief Gets Noise through n which acts as a seed number.
    /// @param n
    /// @return
    fn get_noise_i(&self, mut n: i32) -> f64 {
        n += self.seed;
        n = (n << 13) ^ n;
        let new_n = (n * (n * n * 60493 + 19990303) + 1376312589) & 0x7fffffff;
        1.0 - new_n as f64 / 1073741824.0
    }

    /// @brief Overload of getNoise that takes doubles instead of int n.
    /// @param x
    /// @param z
    /// @return
    fn get_noise_dd(&self, x: f64, z: f64) -> f64 {
        self.get_noise_i((x + z * 57.0) as i32)
    }

    fn lerp(a: f64, b: f64, z: f64) -> f64 {
        let mu2 = (1.0 - (z * 3.14).cos()) / 2.0;
        a * (1.0 - mu2) + b * mu2
    }

    fn noise(&self, x: f64, z: f64) -> f64 {
        let floor_x = x.floor();
        let floor_z = z.floor();

        let s = self.get_noise_dd(floor_x, floor_z);
        let t = self.get_noise_dd(floor_x + 1.0, floor_z);
        let u = self.get_noise_dd(floor_x, floor_z + 1.0); // Get the surrounding values to calculate the transition.
        let v = self.get_noise_dd(floor_x + 1.0, floor_z + 1.0);

        let rec1 = Self::lerp(s, t, x - floor_x); // Interpolate between the values.
        let rec2 = Self::lerp(
            u, v,
            x - floor_x // Here we use x-floorX, to get 1st dimension. Don't mind
        );                 // the x-floorX thingie, it's part of the cosine formula.
        let rec3 = Self::lerp(rec1, rec2, z - floor_z); // Here we use y-floorZ, to get the 2nd dimension.

        rec3
    }
}