use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
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

    /// Reinterpret the bytes at this pointer as static reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid , and will remain valid for
    /// the rest of the program.
    pub unsafe fn as_static(&self) -> &'static T {
        (self.ptr as *const T).as_ref().expect("Failed to dereference UnsafeRef, perhaps the value has been dropped.")
    }

    /// Reinterpret the bytes at this pointer as static mutable reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid , and will remain valid for
    /// the rest of the program.
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
pub struct HeapNullable<T> {
    ptr: *mut T,
    drop: bool
}

impl<T> HeapNullable<T> {
    pub fn new(value: T) -> HeapNullable<T> {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            HeapNullable {
                ptr,
                drop: true
            }
        }
    }

    pub fn null() -> HeapNullable<T> {
        HeapNullable {
            ptr: std::ptr::null_mut(),
            drop: false
        }
    }

    pub fn zeroed() -> HeapNullable<T> {
        unsafe {
            HeapNullable {
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

    pub fn take_replace(&mut self, value: T) -> T {
        unsafe {
            if self.ptr.is_null() {
                panic!("Null pointer dereference!")
            } else {
                let val = self.ptr.read();
                self.ptr.write(value);
                val
            }
        }
    }

    pub fn take_replace_null(&mut self) -> T {
        unsafe {
            if !self.ptr.is_null() {
                let val = self.ptr.read();
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
                self.ptr = std::ptr::null_mut();
                self.drop = false;
                val
            }
            else {
                panic!("Null pointer dereference!")
            }
        }
    }

    pub fn take_replace_zeroed(&mut self) -> T {
        unsafe {
            if self.ptr.is_null() {
                panic!("Null pointer dereference!")
            } else {
                let val = self.ptr.read();
                self.ptr.write_bytes(0, Layout::new::<T>().size());
                val
            }
        }
    }

    pub fn extract(self) -> T {
        unsafe {
            std::ptr::read(self.ptr)
        }
    }

    /// Leaks the [`HeapNullable<T>`], returning the pointer to the heap allocated value.
    ///
    /// # Safety
    ///
    /// This will return a null pointer if the [`HeapNullable<T>`] is null.
    pub unsafe fn leak(mut self) -> *mut T {
        self.drop = false;
        self.ptr
    }

    /// Reinterpret the value at this pointer as another type. This does not cast, it just assumes
    /// the bytes at the pointer are the same type and same length and alignment.
    ///
    /// # Safety
    ///
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] have the same size and alignment.
    pub unsafe fn cast_bytes<R>(self) -> HeapNullable<R> {
        unsafe {
            (&self as *const Self).cast_mut().as_mut().unwrap().drop = false;
        }
        HeapNullable {
            ptr: self.ptr as *mut R,
            drop: true
        }
    }
}

impl<T> Deref for HeapNullable<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().expect("Null pointer dereference! Check using HeapNullable::is_null() before dereferencing!") }
    }
}

impl<T> DerefMut for HeapNullable<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().expect("Null pointer dereference! Check using HeapNullable::is_null() before dereferencing!") }
    }
}

impl<T> Drop for HeapNullable<T> {
    fn drop(&mut self) {
        unsafe {
            if self.drop && !self.ptr.is_null() {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::new::<T>());
            }
        }
    }
}

impl<T: Display> Display for HeapNullable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            f.write_str("null")
        }
        else {
            self.deref().fmt(f)
        }
    }
}

impl<T: PartialEq> PartialEq for HeapNullable<T> {
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

impl<T: Eq> Eq for HeapNullable<T> {}

impl<T> From<T> for HeapNullable<T> {
    fn from(value: T) -> HeapNullable<T> {
        HeapNullable::new(value)
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Nullable<T> {
    val: Option<T>,
}

impl<T> Nullable<T> {
    pub fn new(value: T) -> Nullable<T> {
        Nullable {
            val: Some(value)
        }
    }

    pub fn null() -> Nullable<T> {
        Nullable {
            val: None
        }
    }

    pub fn zeroed() -> Nullable<T> {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T;
            let val = ptr.read();
            std::alloc::dealloc(ptr as *mut u8, Layout::new::<T>());
            Nullable {
                val: Some(val)
            }
        }
    }

    pub fn is_null(&self) -> bool {
        self.val.is_none()
    }

    pub fn replace(&mut self, value: T) {
        self.val.replace(value);
    }

    pub fn replace_null(&mut self) {
        self.val.take();
    }

    pub fn replace_zeroed(&mut self) {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T;
            let val = ptr.read();
            std::alloc::dealloc(ptr as *mut u8, Layout::new::<T>());
            self.val.replace(val);
        }
    }

    pub fn take_replace(&mut self, value: T) -> T {
        self.val.replace(value).expect("Null pointer dereference!")
    }

    pub fn take_replace_null(&mut self) -> T {
        self.val.take().expect("Null pointer dereference!")
    }

    pub fn take_replace_zeroed(&mut self) -> T {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *mut T;
            let val = ptr.read();
            std::alloc::dealloc(ptr as *mut u8, Layout::new::<T>());
            self.val.replace(val).expect("Null pointer dereference!")
        }
    }

    pub fn extract(self) -> T {
        self.val.expect("Extracting a null value!")
    }
}

impl<T> Deref for Nullable<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.val.as_ref().expect("Null pointer dereference! Check using StackNullable::is_null() before dereferencing!")
    }
}

impl<T> DerefMut for Nullable<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.val.as_mut().expect("Null pointer dereference! Check using StackNullable::is_null() before dereferencing!")
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

impl<T> From<T> for Nullable<T> {
    fn from(value: T) -> Nullable<T> {
        Nullable::new(value)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(value: Option<T>) -> Self {
        Nullable {
            val: value,
        }
    }
}

pub struct Unsafe;

impl Unsafe {

    /// Reinterpret the bytes at this pointer as another type. This does not change the bytes at the
    /// reference.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] are of the same size and alignment.
    pub unsafe fn cast_ref<T, R>(value: &T) -> &R {
        (value as *const T as *const R).as_ref().unwrap()
    }

    /// Reinterpret the bytes at this mutable pointer as another type. This does not change the bytes at the
    /// reference.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] are of the same size and alignment.
    pub unsafe fn cast_mut<T, R>(value: &mut T) -> &mut R {
        (value as *mut T as *mut R).as_mut().unwrap()
    }

    /// Reinterpret the bytes at this pointer as static reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid , and will remain valid for
    /// the rest of the program.
    pub unsafe fn cast_static<T>(value: &T) -> &'static T {
        (value as *const T).as_ref().unwrap()
    }

    /// Reinterpret the bytes at this mutable pointer as static reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid , and will remain valid for
    /// the rest of the program.
    pub unsafe fn cast_mut_static<T>(value: &mut T) -> &'static mut T {
        (value as *mut T).as_mut().unwrap()
    }

    /// Move the value to the heap and keep it alive for the rest of the program. Returning a reference
    /// to the value.
    pub fn leak<T>(value: T) -> &'static T {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            ptr.as_ref().unwrap()
        }
    }

    /// Move the value to the heap and keep it alive for the rest of the program. Returning a mutable reference
    /// to the value.
    pub fn leak_mut<T>(value: T) -> &'static mut T {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            ptr.write(value);
            ptr.as_mut().unwrap()
        }
    }

    /// Allocate a zeroed value on the heap and return a reference to it of type [`T`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the type [`T`] is valid with the zeroed data,
    /// or that the data is added before handing this reference to other parts of the program.
    pub unsafe fn leak_zeroed<T>() -> &'static T {
        unsafe {
            let ptr = std::alloc::alloc_zeroed(Layout::new::<T>()) as *const T;
            ptr.as_ref().unwrap()
        }
    }

    /// Allocate a zeroed value on the heap and return a mutable reference to it of type [`T`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the type [`T`] is valid with the zeroed data,
    /// or that the data is added before handing this reference to other parts of the program.
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

impl<T> From<T> for UnsafeRc<T> {
    fn from(value: T) -> Self {
        UnsafeRc::new(value)
    }
}

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

impl<T> From<T> for UnsafeArc<T> {
    fn from(value: T) -> Self {
        UnsafeArc::new(value)
    }
}

unsafe impl<T: Send> Send for UnsafeArc<T> {}

unsafe impl<T: Sync> Sync for UnsafeArc<T> {}

#[repr(transparent)]
pub struct DangerousCell<T> {
    inner: UnsafeCell<T>
}

impl<T> DangerousCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        DangerousCell {
            inner: UnsafeCell::new(value)
        }
    }

    #[inline(always)]
    pub fn get(&self) -> &T {
        unsafe { self.inner.get().as_ref().unwrap() }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { self.inner.get().as_mut().unwrap() }
    }

    #[inline(always)]
    pub fn replace(&self, value: T) {
        unsafe { self.inner.get().write(value); }
    }
}

impl<T: Copy> DangerousCell<T> {
    #[inline(always)]
    pub fn get_val(&self) -> T {
        unsafe { *self.inner.get() }
    }
}

impl<T> From<T> for DangerousCell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

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