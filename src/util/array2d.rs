/*
TODO: There's some bug with the const generic parameters within Rust compiler.

When adding `const WIDTH: usize` to the generic list, the following error occurs:

error: unconstrained generic constant
 --> src/util/array2d.rs:3:12
  |
3 |     array: [T; WIDTH * WIDTH]
  |            ^^^^^^^^^^^^^^^^^^
  |
  = help: try adding a `where` bound using this expression: `where [(); WIDTH * WIDTH]:`

Hence the temporary solution is to switch the implementation to the version based on
heap memory allocation, using Vec to simulate an "array".
 */

#[derive(Clone)]
pub struct Array2D<T: Ord + Clone> {
    array: Vec<T>,
    width: usize
}

impl<T: Ord + Clone> Array2D<T> {
    pub fn new(width: usize) -> Self {
        Self {
            array: Vec::with_capacity(width * width),
            width
        }
    }

    pub fn get(&self, x: usize, z: usize) -> &T {
        &self.array[x * self.width + z]
    }
    
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        &mut self.array[x * self.width + z]
    }
    
    pub fn get_max_value(&self) -> &T {
        self.array.iter().max().unwrap()
    }
    
    pub fn set_all(&mut self, val: T) {
        self.array.fill(val)
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
}