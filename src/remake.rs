use std::ops::{Deref, DerefMut};
use crate::save::{Loader, Savable, Saver};

pub struct Remake<T> {
    item: Option<T>,
}

impl<T> Remake<T> {
    pub fn new(item: T) -> Self {
        Remake { item: Some(item) }
    }

    pub fn get(&self) -> &T {
        self.item
            .as_ref()
            .expect("Remake item should never be None")
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.item
            .as_mut()
            .expect("Remake item should never be None")
    }

    pub fn take(self) -> T {
        self.item.expect("Remake item should never be None")
    }

    pub fn replace<F: FnOnce(T) -> T>(&mut self, function: F) {
        self.item = Some(function(
            self.item.take().expect("Remake item should never be None"),
        ));
    }
}

impl<T> From<T> for Remake<T> {
    fn from(value: T) -> Self {
        Remake { item: Some(value) }
    }
}

impl<T> Deref for Remake<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for Remake<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: Savable> Savable for Remake<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.get().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(Remake::new(T::load(loader)?))
    }
}
