use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct UnsafeCellWrapper<T: ?Sized>(UnsafeCell<T>);

impl<T> UnsafeCellWrapper<T> {
    pub fn new(field0: T) -> Self {
        Self(UnsafeCell::new(field0))
    }
}

impl<T: ?Sized> Deref for UnsafeCellWrapper<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for UnsafeCellWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<T: ?Sized> Send for UnsafeCellWrapper<T> {}

unsafe impl<T: ?Sized> Sync for UnsafeCellWrapper<T> {}