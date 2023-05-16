use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub union UnsafeRef<T> {
    ptr: *mut c_void,
    phantom: PhantomData<T>,
}

impl<T> UnsafeRef<T> {
    pub unsafe fn new(data: &T) -> Self {
        Self {
            ptr: data as *const T as *mut c_void,
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            (self.ptr as *const T).as_ref().is_some()
        }
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            (self.ptr as *const T).as_ref().expect("Failed to dereference UnsafeRef, perhaps the value has been dropped.")
        }
    }
}

impl<T> DerefMut for UnsafeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            (self.ptr as *mut T).as_mut().expect("Failed to dereference UnsafeRef, perhaps the value has been dropped.")
        }
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                ptr: self.ptr
            }
        }
    }
}

impl<T: Display> Display for UnsafeRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: Debug> Debug for UnsafeRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}