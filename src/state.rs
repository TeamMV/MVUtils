use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::unsafe_utils::DangerousCell;

pub struct State<T> {
    inner: Arc<(DangerousCell<u64>, RwLock<T>)>,
    local_version: DangerousCell<u64>,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new((DangerousCell::new(0), RwLock::new(value))),
            local_version: DangerousCell::new(0),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.1.read()
    }

    pub fn write(&self) -> StateWriteGuard<T> {
        StateWriteGuard {
            inner: self.inner.1.write(),
            ptr: self.inner.0.get_mut(),
        }
    }

    pub fn get_version(&self) -> u64 {
        self.inner.0.get_val()
    }

    pub fn get_local_version(&self) -> u64 {
        self.local_version.get_val()
    }

    pub fn is_outdated(&self) -> bool {
        self.inner.0.get_val() != self.local_version.get_val()
    }

    pub fn update(&self) {
        self.local_version.replace(self.inner.0.get_val());
    }

    pub fn force_outdated(&self) {
        if self.inner.0.get_val() == 0 {
            self.local_version.replace(u64::MAX);
        } else {
            self.local_version.replace(0);
        }
    }

    pub fn map<U>(&self, mapper: fn(&T) -> U) -> MappedState<T, U> {
        MappedState::new(mapper, self.clone())
    }
}

impl<T: Clone> State<T> {
    pub fn map_identity(&self) -> MappedState<T, T> {
        MappedState::new(|x| x.clone(), self.clone())
    }
}

unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            inner: self.inner.clone(),
            local_version: DangerousCell::new(self.local_version.get_val()),
        }
    }
}

impl<T> PartialEq for State<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Eq for State<T> {}

impl<T> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for State<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.local_version.get_val().cmp(&other.local_version.get_val())
    }
}

pub struct StateWriteGuard<'a, T: ?Sized + 'a> {
    inner: RwLockWriteGuard<'a, T>,
    ptr: &'a mut u64,
}

impl<'a, T: ?Sized + 'a> Deref for StateWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for StateWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T: ?Sized + 'a> Drop for StateWriteGuard<'a, T> {
    fn drop(&mut self) {
        *self.ptr += 1;
    }
}

#[macro_export]
macro_rules! when {
    ([$($dependency:expr),+$(,)?] => $code:block) => {
        if $($dependency.is_outdated())||+ $code
    };
    ([$($dependency:expr),+$(,)?] => $code:block else $otherwise:block) => {
        if $($dependency.is_outdated())||+ $code
        else $otherwise
    };
    ([] => $code:block) => {};
    ([] => $code:block else $otherwise:block) => { $otherwise };
}

#[macro_export]
macro_rules! update {
    ([$($dependency:expr),+$(,)?]) => {
        $(
            $dependency.update();
        )+
    };
    ([]) => {};
}

#[derive(Clone)]
pub struct MappedState<T, U> {
    mapper: fn(&T) -> U,
    old: State<T>,
}

impl<T, U> MappedState<T, U> {
    pub fn new(mapper: fn(&T) -> U, state: State<T>) -> Self {
        Self {
            mapper,
            old: state,
        }
    }

    pub fn read(&self) -> MappedStateReadGuard<'_, T, U> {
        let guard = self.old.read();
        MappedStateReadGuard {
            mapped: (self.mapper)(guard.deref()),
            rwlock_guard: guard,
        }
    }

    pub fn write(&self) -> StateWriteGuard<T> {
        StateWriteGuard {
            inner: self.old.inner.1.write(),
            ptr: self.old.inner.0.get_mut(),
        }
    }

    pub fn get_version(&self) -> u64 {
        self.old.inner.0.get_val()
    }

    pub fn get_local_version(&self) -> u64 {
        self.old.local_version.get_val()
    }

    pub fn is_outdated(&self) -> bool {
        self.old.inner.0.get_val() != self.old.local_version.get_val()
    }

    pub fn update(&self) {
        self.old.local_version.replace(self.old.inner.0.get_val());
    }

    pub fn force_outdated(&self) {
        if self.old.inner.0.get_val() == 0 {
            self.old.local_version.replace(u64::MAX);
        } else {
            self.old.local_version.replace(0);
        }
    }
}

pub struct MappedStateReadGuard<'a, T, U> {
    mapped: U,
    rwlock_guard: RwLockReadGuard<'a, T>
}

impl<'a, T, U> Deref for MappedStateReadGuard<'a, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.mapped
    }
}