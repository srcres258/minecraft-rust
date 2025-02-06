use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct UnsafeWrappedRef<T> {
    ptr: *const T
}

#[derive(Copy, Clone)]
pub struct UnsafeWrappedRefMut<T> {
    ptr: *mut T
}

pub type UWRef<T> = UnsafeWrappedRef<T>;
pub type UWRefMut<T> = UnsafeWrappedRefMut<T>;

impl<T> UnsafeWrappedRef<T> {
    pub fn from_ref(reference: &T) -> Self {
        Self {
            ptr: reference as *const T
        }
    }
    pub fn from_ptr(ptr: *const T) -> Self {
        Self { ptr }
    }
    pub fn from_uw_ref_mut(uw: UnsafeWrappedRefMut<T>) -> Self {
        Self {
            ptr: uw.ptr()
        }
    }

    pub fn ptr(&self) -> *const T {
        self.ptr
    }

    pub fn drop_box(&self) {
        unsafe {
            let box_obj = Box::from_raw(self.ptr as *mut T);
            drop(box_obj);
        }
    }
    pub fn drop_rc(&self) {
        unsafe {
            let box_obj = Rc::from_raw(self.ptr as *mut T);
            drop(box_obj);
        }
    }
    pub fn drop_arc(&self) {
        unsafe {
            let box_obj = Arc::from_raw(self.ptr);
            drop(box_obj);
        }
    }
}

impl<T> Deref for UnsafeWrappedRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> From<UnsafeWrappedRefMut<T>> for UnsafeWrappedRef<T> {
    fn from(value: UnsafeWrappedRefMut<T>) -> Self {
        Self::from_uw_ref_mut(value)
    }
}

impl<T> UnsafeWrappedRefMut<T> {
    pub fn from_ref(reference: &mut T) -> Self {
        Self {
            ptr: reference as *mut T
        }
    }
    pub fn from_ref_const(reference: &T) -> Self {
        Self {
            ptr: reference as *const T as *mut T
        }
    }
    pub fn from_ptr(ptr: *mut T) -> Self {
        Self { ptr }
    }
    pub fn from_ptr_const(ptr: *const T) -> Self {
        Self {
            ptr: ptr as *mut T
        }
    }
    pub fn from_uw_ref(uw: UnsafeWrappedRef<T>) -> Self {
        Self {
            ptr: uw.ptr() as *mut T
        }
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn drop_box(&self) {
        unsafe {
            let box_obj = Box::from_raw(self.ptr);
            drop(box_obj);
        }
    }
    pub fn drop_rc(&self) {
        unsafe {
            let box_obj = Rc::from_raw(self.ptr);
            drop(box_obj);
        }
    }
    pub fn drop_arc(&self) {
        unsafe {
            let box_obj = Arc::from_raw(self.ptr);
            drop(box_obj);
        }
    }
}

impl<T> Deref for UnsafeWrappedRefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> DerefMut for UnsafeWrappedRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.ptr
        }
    }
}

impl<T> From<UnsafeWrappedRef<T>> for UnsafeWrappedRefMut<T> {
    fn from(value: UnsafeWrappedRef<T>) -> Self {
        Self::from_uw_ref(value)
    }
}

pub fn uw_ref<T>(r: &T) -> UWRef<T> {
    UWRef::from_ref(r)
}
pub fn uw_ref_ptr<T>(r: *const T) -> UWRef<T> {
    UWRef::from_ptr(r)
}

pub fn uw_ref_mut<T>(r: &mut T) -> UWRefMut<T> {
    UWRefMut::from_ref(r)
}
pub fn uw_ref_mut_ptr<T>(r: *mut T) -> UWRefMut<T> {
    UWRefMut::from_ptr(r)
}

unsafe impl<T> Sync for UnsafeWrappedRef<T> {}
unsafe impl<T> Sync for UnsafeWrappedRefMut<T> {}