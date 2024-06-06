#[derive(Default)]
pub struct Array2D<T: Ord, const WIDTH: usize> {
    array: [T; WIDTH * WIDTH]
}

impl<T: Ord, const WIDTH: usize> Array2D<T, WIDTH> {
    pub fn get(&self, x: usize, z: usize) -> &T {
        &self.array[x * WIDTH + z]
    }
    
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        &mut self.array[x * WIDTH + z]
    }
    
    pub fn get_max_value(&self) -> &T {
        self.array.iter().max().unwrap()
    }
    
    pub fn set_all(&mut self, val: T) {
        self.array.fill(val)
    }
}