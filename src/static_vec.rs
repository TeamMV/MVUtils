use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use crate::unsafe_utils::Nullable;

pub struct StaticVec<T> {
    vec: Vec<Nullable<T>>,
    len: usize,
}

impl<T: Default + Clone> StaticVec<T> {
    pub fn new_default(len: usize) -> Self {
        vec![T::default(); len].into()
    }
}

impl<T> StaticVec<T> {
    pub fn new(len: usize) -> Self {
        StaticVec {
            vec: vec![0; len].into_iter().map(|_| Nullable::null()).collect(),
            len
        }
    }

    pub fn as_ptr(&self) -> *const Nullable<T> {
        self.vec.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut Nullable<T> {
        self.vec.as_mut_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&Nullable<T>> {
        self.vec.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Nullable<T>> {
        self.vec.get_mut(index)
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        self.vec[index] = Nullable::new(value);
    }

    pub fn iter(&self) -> Iter<Nullable<T>> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Nullable<T>> {
        self.vec.iter_mut()
    }
}

impl<T> IntoIterator for StaticVec<T> {
    type Item = Nullable<T>;
    type IntoIter = IntoIter<Nullable<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a StaticVec<T> {
    type Item = &'a Nullable<T>;
    type IntoIter = Iter<'a, Nullable<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut StaticVec<T> {
    type Item = &'a mut Nullable<T>;
    type IntoIter = IterMut<'a, Nullable<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> FromIterator<T> for StaticVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let vec = iter.into_iter().map(|v| Nullable::new(v)).collect::<Vec<_>>();
        let len = vec.len();
        StaticVec {
            vec,
            len,
        }
    }
}

impl<T> FromIterator<Nullable<T>> for StaticVec<T> {
    fn from_iter<I: IntoIterator<Item=Nullable<T>>>(iter: I) -> Self {
        let vec = iter.into_iter().collect::<Vec<_>>();
        let len = vec.len();
        StaticVec {
            vec,
            len,
        }
    }
}

impl<T> From<Vec<T>> for StaticVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let vec = vec.into_iter().map(|v| Nullable::new(v)).collect::<Vec<_>>();
        let len = vec.len();
        StaticVec {
            vec,
            len,
        }
    }
}

impl<T> From<Vec<Nullable<T>>> for StaticVec<T> {
    fn from(vec: Vec<Nullable<T>>) -> Self {
        let vec = vec.into_iter().collect::<Vec<_>>();
        let len = vec.len();
        StaticVec {
            vec,
            len,
        }
    }
}

impl<T: Clone> From<&[T]> for StaticVec<T> {
    fn from(slice: &[T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = Nullable::new(t.clone());
        }
        vec
    }
}

impl<T: Clone> From<&mut [T]> for StaticVec<T> {
    fn from(slice: &mut [T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = Nullable::new(t.clone());
        }
        vec
    }
}

impl<T> Deref for StaticVec<T> {
    type Target = [Nullable<T>];

    #[inline]
    fn deref(&self) -> &[Nullable<T>] {
        &self.vec
    }
}

impl<T> DerefMut for StaticVec<T> {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<T> Index<usize> for StaticVec<T> {
    type Output = Nullable<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for StaticVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        &mut self.vec[index]
    }
}

impl<T: Debug> Debug for StaticVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}