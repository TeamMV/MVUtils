use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use crate::once::Lazy;
use crate::utils::Recover;
use hashbrown::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

pub struct ThreadUnique<T> {
    inner: Lazy<Mutex<HashMap<ThreadId, Arc<Mutex<T>>>>>,
    gen: fn() -> T,
}

impl<T> ThreadUnique<T> {
    pub const fn new(gen: fn() -> T) -> Self {
        ThreadUnique {
            inner: Lazy::new(|| HashMap::new().into()),
            gen,
        }
    }

    pub fn get(&self) -> Arc<Mutex<T>> {
        let mut inner = self.inner.lock().recover();
        inner
            .entry(std::thread::current().id())
            .or_insert(Arc::new(Mutex::new((self.gen)()))).clone()
    }
}

impl<T> Deref for ThreadUnique<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.get()
    }
}

impl<T> DerefMut for ThreadUnique<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.get()
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

// TODO: macroize everything lmao
// also make sure to search every trait
impl<T> Deref for ThreadSafe<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ThreadSafe<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Debug> Debug for ThreadSafe<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<T: Default> Default for ThreadSafe<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone> Clone for ThreadSafe<T> {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl<T: PartialEq> PartialEq for ThreadSafe<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.inner, &other.inner)
    }
}

impl<T: Eq> Eq for ThreadSafe<T> {}

impl<T: PartialOrd> PartialOrd for ThreadSafe<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.inner, &other.inner)
    }
}

impl<T: Ord> Ord for ThreadSafe<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&self.inner, &other.inner)
    }
}

impl<T: Hash> Hash for ThreadSafe<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self.inner, state)
    }
}

impl<T: Display> Display for ThreadSafe<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
