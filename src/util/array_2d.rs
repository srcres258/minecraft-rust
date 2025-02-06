use std::ops::DerefMut;
use crate::util::mem::{uw_ref_mut, UWRefMut};

/// @brief Array template used in mathematical calculations.
/// @tparam T
/// @tparam WIDTH
pub struct Array2D<T : Copy + Default + Ord, const WIDTH: usize>
where [(); WIDTH * WIDTH]: {
    array: [T; WIDTH * WIDTH]
}

impl<T : Copy + Default + Ord, const WIDTH: usize> Array2D<T, WIDTH>
where [(); WIDTH * WIDTH]: {
    pub fn new() -> Self {
        Self {
            array: [T::default(); WIDTH * WIDTH]
        }
    }

    pub fn get(&self, x: usize, z: usize) -> &T {
        &self.array[x * WIDTH + z]
    }
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        &mut self.array[x * WIDTH + z]
    }

    pub fn max_value(&self) -> &T {
        let mut result = None;
        for element in self.array.iter() {
            match result {
                Some(r) => {
                    if element > r {
                        result = Some(element);
                    }
                }
                None => result = Some(element)
            }
        }
        result.unwrap()
    }
    pub fn max_value_mut(&mut self) -> UWRefMut<T> {
        let mut result: Option<UWRefMut<T>> = None;
        for element in self.array.iter_mut() {
            match result {
                Some(mut r) => {
                    if element > r.deref_mut() {
                        result = Some(uw_ref_mut(element));
                    }
                }
                None => result = Some(uw_ref_mut(element))
            }
        }
        result.unwrap()
    }
    
    pub fn set_all(&mut self, val: T) {
        self.array.fill(val);
    }
}