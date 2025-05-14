use crate::once::Lazy;
use crate::utils::Recover;
use hashbrown::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use std::thread::ThreadId;

pub struct ThreadUnique<T> {
    inner: Lazy<Mutex<HashMap<ThreadId, T>>>,
    gen: fn() -> T,
}

impl<T> ThreadUnique<T> {
    pub const fn new(gen: fn() -> T) -> Self {
        ThreadUnique {
            inner: Lazy::new(|| HashMap::new().into()),
            gen,
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get(&self) -> &mut T {
        let mut inner = self.inner.lock().recover();
        let ptr = inner
            .entry(std::thread::current().id())
            .or_insert((self.gen)()) as *mut T;
        unsafe { ptr.as_mut().unwrap() }
    }
}

impl<T> Deref for ThreadUnique<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for ThreadUnique<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get()
    }
}

#[macro_export]
macro_rules! thread_unique {
    {
        $(
            $v:vis static $n:ident $($k:ident)?: $t:ty = $init:expr;
        )*
    } => {
        $(
            $v static $n $($k)?: $crate::thread::ThreadUnique<$t> = $crate::thread::ThreadUnique::new(|| { $init });
        )*
    };
    {
        $(
            let $n:ident $($k:ident)?$(: $t:ty)? = $init:expr;
        )*
    } => {
        $(
            let $n $($k)?: $crate::thread::ThreadUnique<$t> = $crate::thread::ThreadUnique::new(|| { $init });
        )*
    };
}

pub struct ThreadSafe<T> {
    inner: T
}

impl<T> ThreadSafe<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn as_ref(&self) -> &T {
        &self.inner
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

unsafe impl<T> Send for ThreadSafe<T> {}
unsafe impl<T> Sync for ThreadSafe<T> {}