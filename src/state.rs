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
    ([$($dependency:ident),+] => $code:block) => {
        if $($dependency.is_outdated())||+ $code
    };
    ([$($dependency:ident),+] => $code:block else $otherwise:block) => {
        if $($dependency.is_outdated())||+ $code
        else $otherwise
    };
}