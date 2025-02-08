use std::sync::Mutex;
use lazy_static::lazy_static;
use mt19937::MT19937;
use rand::Rng;
use rand_core::SeedableRng;
use crate::util::singleton::Singleton;

/// @brief Singleton class that increases randomness (such as with seeds)
pub struct RandomSigleton {
    random_engine: Mutex<MT19937>
}

#[derive(Default)]
pub struct Random {
    random_engine: MT19937
}

lazy_static! {
    static ref RANDOM_SIGLETON_INSTANCE: RandomSigleton = RandomSigleton::new();
}

impl RandomSigleton {
    fn new() -> Self {
        Self {
            random_engine: Mutex::new(MT19937::default())
        }
    }

    pub fn i32_in_range(&self, low: i32, high: i32) -> i32 {
        self.random_engine.lock().unwrap().random_range(low ..= high)
    }
}

impl Singleton for RandomSigleton {
    fn get() -> &'static Self {
        &RANDOM_SIGLETON_INSTANCE
    }
} 

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            random_engine: MT19937::seed_from_u64(seed)
        }
    }

    pub fn i32_in_range(&mut self, low: i32, high: i32) -> i32 {
        self.random_engine.random_range(low ..= high)
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.random_engine = MT19937::seed_from_u64(seed);
    }
}