use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::vec::IntoIter;

pub struct StaticVec<T> {
    vec: Vec<Option<T>>,
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
            vec: vec![0; len].into_iter().map(|_| None).collect(),
            len,
        }
    }

    pub fn as_ptr(&self) -> *const Option<T> {
        self.vec.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut Option<T> {
        self.vec.as_mut_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index).map(|a| a.as_ref()).unwrap_or(None)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index).map(|a| a.as_mut()).unwrap_or(None)
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len {
            panic!("Index {index} out of bounds for length {}!", self.len);
        }
        self.vec[index] = Some(value);
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<&T>> {
        self.vec.iter().map(|t| t.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = Option<&mut T>> {
        self.vec.iter_mut().map(|t| t.as_mut())
    }
}

impl<T> IntoIterator for StaticVec<T> {
    type Item = Option<T>;
    type IntoIter = IntoIter<Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> FromIterator<T> for StaticVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec = iter.into_iter().map(|v| Some(v)).collect::<Vec<_>>();
        let len = vec.len();
        StaticVec { vec, len }
    }
}

impl<T> FromIterator<Option<T>> for StaticVec<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        let vec = iter.into_iter().collect::<Vec<_>>();
        let len = vec.len();
        StaticVec { vec, len }
    }
}

impl<T> From<Vec<T>> for StaticVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let vec = vec.into_iter().map(|v| Some(v)).collect::<Vec<_>>();
        let len = vec.len();
        StaticVec { vec, len }
    }
}

impl<T> From<Vec<Option<T>>> for StaticVec<T> {
    fn from(vec: Vec<Option<T>>) -> Self {
        let vec = vec.into_iter().collect::<Vec<_>>();
        let len = vec.len();
        StaticVec { vec, len }
    }
}

impl<T: Clone> From<&[T]> for StaticVec<T> {
    fn from(slice: &[T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = Some(t.clone());
        }
        vec
    }
}

impl<T: Clone> From<&mut [T]> for StaticVec<T> {
    fn from(slice: &mut [T]) -> Self {
        let len = slice.len();
        let mut vec = StaticVec::new(len);
        for (i, t) in slice.iter().enumerate() {
            vec[i] = Some(t.clone());
        }
        vec
    }
}

impl<T> Deref for StaticVec<T> {
    type Target = [Option<T>];

    #[inline]
    fn deref(&self) -> &[Option<T>] {
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
    type Output = Option<T>;

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
