use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub union UnsafeRef<T> {
    ptr: *mut c_void,
    phantom: PhantomData<T>,
}

impl<T> UnsafeRef<T> {
    /// Initialize an [`UnsafeRef<T>`] with a pointer value.
    ///
    /// # Safety
    ///
    /// It is up to the user to ensure that the pointed to value lives long enough, and that this
    /// struct is dropped when the pointee is.
    pub unsafe fn new(data: &T) -> Self {
        Self {
            ptr: data as *const T as *mut c_void,
        }
    }

    /// Initialize an [`UnsafeRef<T>`] as null.
    ///
    /// # Safety
    ///
    /// This function sets the value to null. It is up to the user to check whether the pointer is
    /// null using [`UnsafeRef::is_null`] before calling functions on the dereferenced value.
    pub unsafe fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    /// Check whether the pointer is null or not
    ///
    /// NOTE: This does not verify whether the pointee has been dropped, there is no way to
    /// do that, which is why this is unsafe
    ///
    /// It does **not** check if the memory is valid, aligned, or initialized.
    pub fn is_null(&self) -> bool {
        unsafe {
            !self.ptr.is_null()
        }
    }

    /// Reinterpret the value at this pointer as another type. This does not cast, it just assumes
    /// the bytes at the pointer are the same type and same length and alignment.
    ///
    /// # Safety
    ///
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] have the same size and alignment, and that no type-specific layout constraints are
    /// violated (e.g., invalid enum tags, misalignment)
    pub unsafe fn cast_bytes<R>(&self) -> UnsafeRef<R> {
        UnsafeRef {
            ptr: self.ptr,
        }
    }

    /// Compare the pointers
    ///
    /// This compares the raw pointers, not the pointee values, true will only be returned if both
    /// refs point to the same memory address
    pub fn same_as<R>(&self, other: &UnsafeRef<R>) -> bool {
        unsafe {
            self.ptr == other.ptr
        }
    }

    /// Reinterpret the bytes at this pointer as a reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// Do not call this unless you know the pointer is not null or have checked using [`is_null`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid.
    #[must_use]
    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        (self.ptr as *const T)
            .as_ref()
            .expect("Failed to dereference UnsafeRef.")
    }

    /// Reinterpret the bytes at this pointer as a mutable reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// Do not call this unless you know the pointer is not null or have checked using [`is_null`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid.
    #[must_use]
    pub unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        (self.ptr as *mut T)
            .as_mut()
            .expect("Failed to dereference UnsafeRef.")
    }
}

unsafe impl<T> UnsafeFrom<&T> for UnsafeRef<T> {
    unsafe fn unsafe_from(value: &T) -> Self {
        Self::new(value)
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                ptr: self.ptr,
            }
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
    #[track_caller]
    #[must_use]
    pub unsafe fn cast_ref<T: Sized, R: Sized>(value: &T) -> &R {
        (value as *const T as *const R).as_ref().unwrap()
    }

    /// Reinterpret the bytes at this mutable pointer as another type. This does not change the bytes at the
    /// reference.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid, and that both types [`T`]
    /// and [`R`] are of the same size and alignment. Additionally, the user must gurantee that
    /// the original reference is not aliased after this is called.
    #[track_caller]
    #[must_use]
    pub unsafe fn cast_mut<T: Sized, R: Sized>(value: &mut T) -> &mut R {
        (value as *mut T as *mut R).as_mut().unwrap()
    }

    /// Reinterpret the bytes at this pointer as static reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid, and will remain valid for
    /// the duration of the new lifetime.
    #[track_caller]
    #[must_use]
    pub unsafe fn cast_lifetime<'a, 'b, T>(value: &'a T) -> &'b T {
        (value as *const T).as_ref().unwrap()
    }

    /// Reinterpret the bytes at this mutable pointer as static reference. This does not change the bytes at the
    /// reference, nor does it prevent them from being dropped.
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the pointer is valid, and will remain valid for
    /// the duration of the new lifetime.
    #[track_caller]
    #[must_use]
    pub unsafe fn cast_lifetime_mut<'a, 'b, T>(value: &'a mut T) -> &'b mut T {
        (value as *mut T).as_mut().unwrap()
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
        Box::leak(Box::new(value))
    }

    /// Move the value to the heap and keep it alive for the rest of the program. Returning a mutable reference
    /// to the value.
    pub fn leak_mut<T>(value: T) -> &'static mut T {
        Box::leak(Box::new(value))
    }

    /// Allocate a zeroed value on the heap and return a reference to it of type [`T`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the type [`T`] is valid with the zeroed data,
    /// or that the data is added before handing this reference to other parts of the program.
    #[track_caller]
    pub unsafe fn leak_zeroed<T>() -> &'static T {
        Box::leak(Box::new(std::mem::MaybeUninit::<T>::zeroed().assume_init()))
    }

    /// Allocate a zeroed value on the heap and return a mutable reference to it of type [`T`].
    ///
    /// # Safety
    /// It is entirely up to the user to ensure that the type [`T`] is valid with the zeroed data,
    /// or that the data is added before handing this reference to other parts of the program.
    #[track_caller]
    pub unsafe fn leak_zeroed_mut<T>() -> &'static mut T {
        Box::leak(Box::new(std::mem::MaybeUninit::<T>::zeroed().assume_init()))
    }
}

#[repr(transparent)]
pub struct DangerousCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> DangerousCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        DangerousCell {
            inner: UnsafeCell::new(value),
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
        unsafe {
            self.inner.get().write(value);
        }
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


// TODO: same macroize needed as thread safe here plz asap when v22 is not rushing me
// also a lot of traits missing
impl<T: Debug> Debug for DangerousCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.get(), f)
    }
}

impl<T: Display> Display for DangerousCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.get(), f)
    }
}

impl<T: Clone> Clone for DangerousCell<T> {
    fn clone(&self) -> Self {
        DangerousCell::new(self.get().clone())
    }
}

impl<T: Default> Default for DangerousCell<T> {
    fn default() -> Self {
        DangerousCell::new(Default::default())
    }
}

/// A copy of the [`From<T>`] trait, but for types where this operation is unsafe.
///
/// # Safety
/// It is up to the user to ensure that when using this, safety checks are implemented.
pub unsafe trait UnsafeFrom<T>: Sized {
    /// Converts to this type from the input type.
    #[must_use]
    unsafe fn unsafe_from(value: T) -> Self;
}

/// A copy of the [`Into<T>`] trait, but for types where this operation is unsafe.
///
/// # Safety
/// It is up to the user to ensure that when using this, safety checks are implemented.
pub unsafe trait UnsafeInto<T>: Sized {
    /// Converts this type into the (usually inferred) input type.
    #[must_use]
    unsafe fn unsafe_into(self) -> T;
}

unsafe impl<T, U> UnsafeInto<U> for T
where
    U: UnsafeFrom<T>,
{
    /// Calls `U::from(self)`.
    ///
    /// That is, this conversion is whatever the implementation of
    /// <code>[UnsafeFrom]&lt;T&gt; for U</code> chooses to do.
    #[inline]
    unsafe fn unsafe_into(self) -> U {
        U::unsafe_from(self)
    }
}

#[macro_export]
macro_rules! unsafe_cast {
    ($val:ident, $to:ty) => {
        unsafe { (($val as *const _) as *const $to).as_ref().unwrap() }
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
