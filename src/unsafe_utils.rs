use std::alloc::Layout;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct UnsafeRef<T> {
    ptr: *mut c_void,
    phantom: PhantomData<T>,
}

impl<T> UnsafeRef<T> {
    pub fn new(data: &T) -> Self {
        Self {
            ptr: data as *const T as *mut c_void,
            phantom: PhantomData
        }
    }

    /// Initialize an [`UnsafeRef<T>`] as null.
    ///
    /// # Safety
    ///
    /// This function sets the value to null. It is up to the user to check whether the pointer is
    /// null using [`UnsafeRef::is_valid`] before calling functions on the dereferenced value.
    pub unsafe fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            phantom: PhantomData
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }

    /// Reinterpret the value at this pointer as another type. This does not cast, it just assumes
    /// the bytes at the pointer are the same type and same length and alignment.
    ///
    /// # Safety
    ///
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] have the same size and alignment.
    pub unsafe fn cast_bytes<R>(&self) -> UnsafeRef<R> {
        UnsafeRef {
            ptr: self.ptr,
            phantom: PhantomData
        }
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

    pub unsafe fn as_static(&self) -> &'static T {
        (self.ptr as *const T).as_ref().expect("Failed to dereference UnsafeRef, perhaps the value has been dropped.")
    }

    pub unsafe fn as_static_mut(&mut self) -> &'static T {
        (self.ptr as *mut T).as_mut().expect("Failed to dereference UnsafeRef, perhaps the value has been dropped.")
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
        Self {
            ptr: self.ptr,
            phantom: PhantomData
        }
    }
}

impl<T: Display> Display for UnsafeRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: Debug> Debug for UnsafeRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for UnsafeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for UnsafeRef<T> {}

#[derive(Debug)]
pub struct Nullable<T> {
    ptr: *mut T,
    drop: bool
}

impl<T> Nullable<T> {
    pub fn new(value: T) -> Nullable<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            Nullable {
                ptr,
                drop: true
            }
        }
    }

    pub fn null() -> Nullable<T> {
        Nullable {
            ptr: std::ptr::null_mut(),
            drop: false
        }
    }

    pub fn zeroed() -> Nullable<T> {
        unsafe {
            Nullable {
                ptr: std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T,
                drop: true
            }
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn replace(&mut self, value: T) {
        unsafe {
            if self.ptr.is_null() {
                let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
                ptr.write(value);
                self.ptr = ptr;
                self.drop = true;
            } else {
                self.ptr.write(value);
            }
        }
    }

    pub fn replace_null(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
                self.ptr = std::ptr::null_mut();
                self.drop = false;
            }
        }
    }

    pub fn replace_zeroed(&mut self) {
        unsafe {
            if self.ptr.is_null() {
                let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T;
                self.ptr = ptr;
                self.drop = true;
            } else {
                self.ptr.write_bytes(0, Layout::new::<T>().size());
            }
        }
    }

    pub fn extract(self) -> T {
        unsafe {
            std::ptr::read(self.ptr)
        }
    }

    /// Leaks the [`Nullable<T>`], returning the pointer to the heap allocated value.
    ///
    /// # Safety
    ///
    /// This will return a null pointer if the [`Nullable<T>`] is null.
    pub unsafe fn leak(self) -> *mut T {
        self.ptr
    }

    /// Reinterpret the value at this pointer as another type. This does not cast, it just assumes
    /// the bytes at the pointer are the same type and same length and alignment.
    ///
    /// # Safety
    ///
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] have the same size and alignment.
    pub unsafe fn cast_bytes<R>(self) -> Nullable<R> {
        unsafe {
            (&self as *const Self).cast_mut().as_mut().unwrap().drop = false;
        }
        Nullable {
            ptr: self.ptr as *mut R,
            drop: true
        }
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
            if self.drop && !self.ptr.is_null() {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
            }
        }
    }
}

impl<T: Display> Display for Nullable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            f.write_str("null")
        }
        else {
            self.deref().fmt(f)
        }
    }
}

impl<T: PartialEq> PartialEq for Nullable<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() {
            other.is_null()
        }
        else if other.is_null() {
            false
        }
        else {
            self.deref() == other.deref()
        }
    }
}

impl<T: Eq> Eq for Nullable<T> {}

#[derive(Debug)]
pub struct NullableRc<T> {
    ptr: *const T,
    ref_count: *mut usize,
}

impl<T> NullableRc<T> {
    pub fn new(value: T) -> NullableRc<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            let ptr = ptr as *const T;
            let ref_count = std::alloc::alloc(Layout::new::<usize>()) as *mut usize;
            ref_count.write(1);
            NullableRc {
                ptr,
                ref_count,
            }
        }
    }

    pub fn null() -> NullableRc<T> {
        unsafe {
            let ref_count = std::alloc::alloc(Layout::new::<usize>()) as *mut usize;
            ref_count.write(1);
            NullableRc {
                ptr: std::ptr::null_mut(),
                ref_count,
            }
        }
    }

    pub fn zeroed() -> NullableRc<T> {
        unsafe {
            let ref_count = std::alloc::alloc(Layout::new::<usize>()) as *mut usize;
            ref_count.write(1);
            NullableRc {
                ptr: std::alloc::alloc_zeroed(Layout::new::<T>()) as *const T,
                ref_count,
            }
        }
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn ref_count(&self) -> usize {
        unsafe { *self.ref_count }
    }

    pub fn extract(self) -> T {
        if self.ref_count() > 1 {
            panic!("Cannot extract a NullableRc with more than one living reference!");
        }
        unsafe {
            std::ptr::read(self.ptr)
        }
    }

    /// Reinterpret the value at this pointer as another type. This does not cast, it just assumes
    /// the bytes at the pointer are the same type and same length and alignment.
    ///
    /// # Safety
    ///
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] have the same size and alignment.
    pub unsafe fn cast_bytes<R>(&self) -> NullableRc<R> {
        *self.ref_count += 1;
        NullableRc {
            ptr: self.ptr as *mut R,
            ref_count: self.ref_count,
        }
    }
}

impl<T> Deref for NullableRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().expect("Null pointer dereference! Check using Nullable::is_null() before dereferencing!") }
    }
}

impl<T> Clone for NullableRc<T> {
    fn clone(&self) -> NullableRc<T> {
        unsafe {
            *self.ref_count += 1;
            NullableRc {
                ptr: self.ptr,
                ref_count: self.ref_count,
            }
        }
    }
}

impl<T> Drop for NullableRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *self.ref_count == 1 {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
                std::alloc::dealloc(self.ref_count as *mut u8, Layout::new::<usize>());
            }
            else {
                *self.ref_count -= 1;
            }
        }
    }
}

impl<T: Display> Display for NullableRc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            f.write_str("null")
        }
        else {
            self.deref().fmt(f)
        }
    }
}

impl<T: PartialEq> PartialEq for NullableRc<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_null() {
            other.is_null()
        }
        else if other.is_null() {
            false
        }
        else {
            self.deref() == other.deref()
        }
    }
}

impl<T: Eq> Eq for NullableRc<T> {}

pub struct Unsafe;

impl Unsafe {
    pub unsafe fn cast<T, R>(value: T) -> R {
        std::ptr::read(&value as *const T as *mut R)
    }

    pub unsafe fn cast_ref<T, R>(value: &T) -> &R {
        (value as *const T as *const R).as_ref().unwrap()
    }

    pub unsafe fn cast_mut<T, R>(value: &mut T) -> &mut R {
        (value as *mut T as *mut R).as_mut().unwrap()
    }

    pub unsafe fn cast_static<T>(value: &T) -> &'static T {
        (value as *const T).as_ref().unwrap()
    }

    pub unsafe fn cast_mut_static<T>(value: &mut T) -> &'static mut T {
        (value as *mut T).as_mut().unwrap()
    }

    pub fn leak<T>(value: T) -> &'static T {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            ptr.as_ref().unwrap()
        }
    }

    pub fn leak_mut<T>(value: T) -> &'static mut T {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            ptr.as_mut().unwrap()
        }
    }

    pub unsafe fn leak_zeroed<T>() -> &'static T {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *const T;
            ptr.as_ref().unwrap()
        }
    }

    pub unsafe fn leak_zeroed_mut<T>() -> &'static mut T {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T;
            ptr.as_mut().unwrap()
        }
    }
}

pub struct UnsafeRc<T> {
    ptr: *const T,
    alloc: bool,
    ref_count: *mut usize,
}

impl<T> UnsafeRc<T> {
    pub fn new(value: T) -> UnsafeRc<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            let ptr = ptr as *const T;
            let ref_count = std::alloc::alloc(Layout::new::<usize>()) as *mut usize;
            ref_count.write(1);
            UnsafeRc {
                ptr,
                alloc: true,
                ref_count,
            }
        }
    }

    pub fn from_ref(value: &T) -> UnsafeRc<T> {
        unsafe {
            let ptr = value as *const T;
            let ref_count = std::alloc::alloc(Layout::new::<usize>()) as *mut usize;
            ref_count.write(1);
            UnsafeRc {
                ptr,
                alloc: false,
                ref_count,
            }
        }
    }
}

impl<T> Clone for UnsafeRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count += 1;
            UnsafeRc {
                ptr: self.ptr,
                alloc: self.alloc,
                ref_count: self.ref_count,
            }
        }
    }
}

impl<T> Drop for UnsafeRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *self.ref_count == 1 {
                if self.alloc {
                    std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
                }
                std::alloc::dealloc(self.ref_count as *mut u8, Layout::new::<usize>());
            }
            else {
                *self.ref_count -= 1;
            }
        }
    }
}

impl<T> Deref for UnsafeRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().expect("Dereferencing a nulled UnsafeRc!") }
    }
}

impl<T: Display> Display for UnsafeRc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for UnsafeRc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for UnsafeRc<T> {}

pub struct UnsafeArc<T> {
    ptr: *const T,
    alloc: bool,
    ref_count: Arc<AtomicUsize>,
}

impl<T> UnsafeArc<T> {
    pub fn new(value: T) -> UnsafeArc<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            let ptr = ptr as *const T;
            let ref_count = Arc::new(AtomicUsize::new(1));
            UnsafeArc {
                ptr,
                alloc: true,
                ref_count,
            }
        }
    }

    pub fn from_ref(value: &T) -> UnsafeArc<T> {
        let ptr = value as *const T;
        let ref_count = Arc::new(AtomicUsize::new(1));
        UnsafeArc {
            ptr,
            alloc: false,
            ref_count,
        }
    }
}

impl<T> Clone for UnsafeArc<T> {
    fn clone(&self) -> Self {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
        UnsafeArc {
            ptr: self.ptr,
            alloc: self.alloc,
            ref_count: self.ref_count.clone(),
        }
    }
}

impl<T> Drop for UnsafeArc<T> {
    fn drop(&mut self) {
        unsafe {
            if self.ref_count.load(Ordering::Relaxed) == 1 && self.alloc {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
            }
            else {
                self.ref_count.fetch_sub(1, Ordering::Relaxed);
            }
        }
    }
}

impl<T> Deref for UnsafeArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().expect("Dereferencing a nulled UnsafeArc!") }
    }
}

impl<T: Display> Display for UnsafeArc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for UnsafeArc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for UnsafeArc<T> {}

unsafe impl<T: Send> Send for UnsafeArc<T> {}

unsafe impl<T: Sync> Sync for UnsafeArc<T> {}

#[macro_export]
macro_rules! unsafe_cast {
    ($val:ident, $to:ty) => {
        unsafe { (($val as *const _) as *const $to).as_ref().unwrap() }
    };
}

#[macro_export]
macro_rules! unsafe_cast_mut {
    ($val:ident, $to:ty) => {
        unsafe { (($val as *const _) as *const $to).cast_mut().as_mut().unwrap() }
    };
}

#[macro_export]
macro_rules! unsafe_multi_borrow {
    ($val:ident, $t:ty) => {
        unsafe { (&$val as *const $t).as_ref().unwrap() }
    };
}

#[macro_export]
macro_rules! unsafe_multi_borrow_mut {
    ($val:ident, $t:ty) => {
        unsafe { (&mut $val as *mut $t).as_mut().unwrap() }
    };
}