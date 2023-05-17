use std::alloc::Layout;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub union UnsafeRef<T> {
    ptr: *mut c_void,
    phantom: PhantomData<T>,
}

impl<T> UnsafeRef<T> {
    pub fn new(data: &T) -> Self {
        Self {
            ptr: data as *const T as *mut c_void,
        }
    }

    pub unsafe fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            (self.ptr as *const T).as_ref().is_some()
        }
    }

    pub unsafe fn cast_bytes<R>(&self) -> UnsafeRef<R> {
        UnsafeRef {
            ptr: self.ptr,
        }
    }
}

impl<T> From<&T> for UnsafeRef<T> {
    fn from(value: &T) -> Self {
        UnsafeRef::new(value)
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

pub struct Nullable<T> {
    ptr: *mut T
}

impl<T> Nullable<T> {
    pub fn new(value: T) -> Nullable<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            Nullable {
                ptr
            }
        }
    }

    pub fn null() -> Nullable<T> {
        Nullable {
            ptr: std::ptr::null_mut()
        }
    }

    pub fn zeroed() -> Nullable<T> {
        unsafe {
            Nullable {
                ptr: std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T
            }
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T> Deref for Nullable<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().expect("Null pointer dereference! Check using Nullable::is_null() before dereferencing!") }
    }
}

impl<T> DerefMut for Nullable<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().expect("Null pointer dereference! Check using Nullable::is_null() before dereferencing!") }
    }
}

impl<T> Drop for Nullable<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }
}
