use std::sync::Mutex;
use std::time::SystemTime;
use lazy_static::lazy_static;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::StdRng;
use rand::Rng;
use rand_core::SeedableRng;

/// @brief Singleton class that increases randomness (such as with seeds)
pub struct RandomSingleton {
    random_engine: Mutex<StdRng>
}

pub struct Random {
    random_engine: Mutex<StdRng>
}

lazy_static! {
    static ref INSTANCE: RandomSingleton = RandomSingleton::new();
}

impl RandomSingleton {
    fn new() -> Self {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Self {
            random_engine: Mutex::new(StdRng::seed_from_u64(duration.as_secs()))
        }
    }

    pub fn get() -> &'static Self {
        &INSTANCE
    }

    pub fn int_in_range<T: SampleUniform>(&self, range: impl SampleRange<T>) -> T {
        self.random_engine.lock().unwrap().gen_range(range)
    }
}

impl Random {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            random_engine: Mutex::new(StdRng::seed_from_u64(seed))
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.random_engine = Mutex::new(StdRng::seed_from_u64(seed));
    }
    
    pub fn int_in_range<T: SampleUniform>(&self, range: impl SampleRange<T>) -> T {
        self.random_engine.lock().unwrap().gen_range(range)
    }
}

impl Default for Random {
    fn default() -> Self {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Self::new_with_seed(duration.as_secs())
    }
}