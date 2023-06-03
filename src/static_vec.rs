use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;
use std::slice;

pub struct StaticVec<T> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T> StaticVec<T> {
    pub fn new(len: usize) -> Self {
        unsafe {
            let ptr = alloc_zeroed(Layout::array::<T>(len).unwrap_or_else(|_| panic!("Length of {len} not valid!"))) as *mut T;
            Self {
                ptr: NonNull::new(ptr).expect("Failed to allocate memory for StaticVec!"),
                len,
            }
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.ptr.as_ptr().add(index).as_ref().unwrap() })
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.ptr.as_ptr().add(index).as_mut().unwrap() })
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len {
            panic!("Index out of bounds!");
        }
        unsafe {
            self.ptr.as_ptr().add(index).write(value);
        }
    }

    pub fn iter(&self) -> StaticVecIter<T> {
        StaticVecIter {
            vec: self,
            index: 0
        }
    }

    pub fn iter_mut(&mut self) -> StaticVecIterMut<T> {
        StaticVecIterMut {
            vec: self,
            index: 0
        }
    }
}

impl<T> IntoIterator for StaticVec<T> {
    type Item = T;
    type IntoIter = StaticVecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StaticVecIntoIter {
            vec: self,
            index: 0
        }
    }
}

impl<'a, T> IntoIterator for &'a StaticVec<T> {
    type Item = &'a T;
    type IntoIter = StaticVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut StaticVec<T> {
    type Item = &'a mut T;
    type IntoIter = StaticVecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> FromIterator<T> for StaticVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let vec = iter.into_iter().collect::<Vec<_>>();
        let len = vec.len();
        StaticVec {
            ptr: NonNull::new(vec.leak().as_mut_ptr()).expect("Failed to collect into StaticVec!"),
            len,
        }
    }
}

impl<T> From<Vec<T>> for StaticVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let len = vec.len();
        StaticVec {
            ptr: NonNull::new(vec.leak().as_mut_ptr()).expect("Failed to collect into StaticVec!"),
            len,
        }
    }
}

impl<T: Clone> From<&[T]> for StaticVec<T> {
    fn from(slice: &[T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = t.clone();
        }
        vec
    }
}

impl<T: Clone> From<&mut [T]> for StaticVec<T> {
    fn from(slice: &mut [T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = t.clone();
        }
        vec
    }
}

impl<T> Deref for StaticVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for StaticVec<T> {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

impl<T> Index<usize> for StaticVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        unsafe { self.ptr.as_ptr().add(index).as_ref().unwrap() }
    }
}

impl<T> IndexMut<usize> for StaticVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        unsafe { self.ptr.as_ptr().add(index).as_mut().unwrap() }
    }
}

impl<T: Debug> Debug for StaticVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T> Drop for StaticVec<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.len).unwrap_or_else(|_| panic!("Length of {} not valid!", self.len)));
        }
    }
}

pub struct StaticVecIter<'a, T> {
    vec: &'a StaticVec<T>,
    index: usize
}

impl<'a, T> Iterator for StaticVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.vec.get(self.index);
        self.index += 1;
        next
    }
}

pub struct StaticVecIterMut<'a, T> {
    vec: &'a mut StaticVec<T>,
    index: usize
}

impl<'a, T> Iterator for StaticVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len {
            return None;
        }
        let next = &mut self.vec[self.index];
        self.index += 1;
        Some(unsafe { (next as *mut T).as_mut().unwrap() })
    }
}

pub struct StaticVecIntoIter<T> {
    vec: StaticVec<T>,
    index: usize
}

impl<T> Iterator for StaticVecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.vec.len {
            None
        }
        else {
            unsafe {
                let next = self.vec.ptr.as_ptr().add(self.index).read();
                self.index += 1;
                Some(next)
            }
        }
    }
}