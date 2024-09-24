use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub struct OrDefault<T> {
    inner: Option<T>,
    def: T
}

impl<T> OrDefault<T> {
    pub fn new(t: T, def: T) -> OrDefault<T> {
        Self {
            inner: Some(t),
            def,
        }
    }

    pub fn uninit(def: T) -> OrDefault<T> {
        Self {
            inner: None,
            def,
        }
    }

    pub fn take(self) -> T {
        self.inner.unwrap_or(self.def)
    }

    pub fn get(&self) -> &T {
        self.inner.as_ref().unwrap_or(&self.def)
    }

    pub fn get_mut(&mut self) -> &mut T {
        let mut am = self.inner.as_mut();
        am.unwrap_or(&mut self.def)
    }

    pub fn set(&mut self, t: T) {
        self.inner = Some(t);
    }
}

impl<T: Default> OrDefault<T> {
    pub fn new_default(t: T) -> OrDefault<T> {
        Self {
            inner: Some(t),
            def: T::default(),
        }
    }

    pub fn uninit_default() -> OrDefault<T> {
        Self {
            inner: None,
            def: T::default(),
        }
    }
}

impl<T> Deref for OrDefault<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for OrDefault<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: Default> Default for OrDefault<T> {
    fn default() -> Self {
        Self {
            inner: Some(T::default()),
            def: T::default(),
        }
    }
}

impl<T: Display> Display for OrDefault<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}