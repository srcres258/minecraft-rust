#[derive(Copy, Clone, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct VectorXZ {
    pub x: i32,
    pub z: i32
}

impl VectorXZ {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z
        }
    }
}