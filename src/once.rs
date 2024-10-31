use std::any::Any;
use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::panic;
use std::panic::{catch_unwind, RefUnwindSafe, UnwindSafe};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, Once,
};
use crate::save::{Loader, Savable, Saver};

#[derive(Debug, Default)]
pub struct AlreadyInitialized;

impl Display for AlreadyInitialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Value already initialized!")
    }
}

impl Error for AlreadyInitialized {}

pub enum InitError {
    AlreadyInitialized(AlreadyInitialized),
    Panicked(Box<dyn Any + Send + 'static>),
}

pub struct InitOnce<T> {
    value: UnsafeCell<T>,
    once: Once,
    init_called: AtomicBool,
}

impl<T> InitOnce<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            once: Once::new(),
            init_called: AtomicBool::new(false),
        }
    }

    pub fn initialized(&self) -> bool {
        self.init_called.load(Ordering::Relaxed)
    }

    pub fn init<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("InitOnce::init called twice");
        }

        self.once.call_once(|| {
            let value = unsafe { &mut *self.value.get() };
            f(value);
        });
    }

    pub fn safe_init<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>>
    where
        F: FnOnce(&mut T) + UnwindSafe,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("InitOnce::init called twice");
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));
        let clone = panicked.clone();

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = unsafe { &mut *self.value.get() };
                f(value);
            });

            if let Err(e) = result {
                clone.lock().unwrap().replace(Err(e));
            }
        });
        let res = panicked.lock().unwrap().take().unwrap();
        res
    }

    pub fn try_init<F>(&self, f: F) -> Result<(), AlreadyInitialized>
    where
        F: FnOnce(&mut T),
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(AlreadyInitialized);
        }

        self.once.call_once(|| {
            let value = unsafe { &mut *self.value.get() };
            f(value);
        });

        Ok(())
    }

    pub fn try_safe_init<F>(&self, f: F) -> Result<(), InitError>
    where
        F: FnOnce(&mut T) + UnwindSafe,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(InitError::AlreadyInitialized(AlreadyInitialized));
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = unsafe { &mut *self.value.get() };
                f(value);
            });

            if let Err(e) = result {
                panicked
                    .lock()
                    .unwrap()
                    .replace(Err(InitError::Panicked(e)));
            }
        });
        let res = panicked.lock().unwrap().take().unwrap();
        res
    }
}

impl<T> Deref for InitOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if !self.init_called.load(Ordering::Relaxed) {
            panic!("InitOnce::deref called before InitOnce::init");
        }
        unsafe { &*self.value.get() }
    }
}

impl<T: Display> Display for InitOnce<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> From<T> for InitOnce<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Savable> Savable for InitOnce<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.value.save(saver);
        self.init_called.load(Ordering::Acquire).save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let value = T::load(loader)?;
        let init_called = AtomicBool::new(bool::load(loader)?);
        Ok(InitOnce {
            value: UnsafeCell::new(value),
            once: Once::new(),
            init_called,
        })
    }
}

unsafe impl<T: Send> Send for InitOnce<T> {}
unsafe impl<T: Sync> Sync for InitOnce<T> {}
impl<T> RefUnwindSafe for InitOnce<T> {}

pub struct CreateOnce<T> {
    value: UnsafeCell<Option<T>>,
    once: Once,
    init_called: AtomicBool,
}

impl<T> CreateOnce<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            once: Once::new(),
            init_called: AtomicBool::new(false),
        }
    }

    pub fn created(&self) -> bool {
        self.init_called.load(Ordering::Relaxed)
    }

    pub fn create<F>(&self, f: F)
    where
        F: FnOnce() -> T,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("CreateOnce::create called twice");
        }

        self.once.call_once(|| {
            let value = f();
            unsafe { &mut *self.value.get() }.replace(value);
        });
    }

    pub fn safe_create<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>>
    where
        F: FnOnce() -> T + UnwindSafe,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("InitOnce::init called twice");
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));
        let clone = panicked.clone();

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = f();
                unsafe { &mut *self.value.get() }.replace(value);
            });

            if let Err(e) = result {
                clone.lock().unwrap().replace(Err(e));
            }
        });
        let res = panicked.lock().unwrap().take().unwrap();
        res
    }

    pub fn try_create<F>(&self, f: F) -> Result<(), AlreadyInitialized>
    where
        F: FnOnce() -> T,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(AlreadyInitialized);
        }

        self.once.call_once(|| {
            let value = f();
            unsafe { &mut *self.value.get() }.replace(value);
        });

        Ok(())
    }

    pub fn try_safe_create<F>(&self, f: F) -> Result<(), InitError>
    where
        F: FnOnce() -> T + UnwindSafe,
    {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(InitError::AlreadyInitialized(AlreadyInitialized));
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = f();
                unsafe { &mut *self.value.get() }.replace(value);
            });

            if let Err(e) = result {
                panicked
                    .lock()
                    .unwrap()
                    .replace(Err(InitError::Panicked(e)));
            }
        });
        let res = panicked.lock().unwrap().take().unwrap();
        res
    }
}

impl<T: Default> CreateOnce<T> {
    pub fn create_default(&self) {
        self.create(T::default);
    }

    pub fn try_create_default(&self) -> Result<(), AlreadyInitialized> {
        self.try_create(T::default)
    }
}

impl<T> Deref for CreateOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if !self.init_called.load(Ordering::Relaxed) {
            panic!("CreateOnce::deref called before CreateOnce::create");
        }

        unsafe { self.value.get().as_ref().unwrap() }
            .as_ref()
            .unwrap()
    }
}

impl<T> DerefMut for CreateOnce<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.init_called.load(Ordering::Relaxed) {
            panic!("CreateOnce::deref called before CreateOnce::create");
        }

        unsafe { self.value.get().as_mut().unwrap() }
            .as_mut()
            .unwrap()
    }
}

impl<T: Display> Display for CreateOnce<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Savable> Savable for CreateOnce<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.value.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let value = <Option<T>>::load(loader)?;
        Ok(CreateOnce {
            init_called: AtomicBool::new(value.is_some()),
            value: UnsafeCell::new(value),
            once: Once::new(),
        })
    }
}

unsafe impl<T: Send> Send for CreateOnce<T> {}
unsafe impl<T: Sync> Sync for CreateOnce<T> {}
impl<T> RefUnwindSafe for CreateOnce<T> {}

pub struct Lazy<T> {
    value: CreateOnce<T>,
    init: Mutex<Option<fn() -> T>>,
}

impl<T> Lazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(f)),
        }
    }

    pub fn created(&self) -> bool {
        self.value.created()
    }
}

impl<T: Default> Lazy<T> {
    pub const fn default() -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(T::default)),
        }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut f = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(f) = f.take() {
            self.value.create(f);
        }
        &self.value
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut f = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(f) = f.take() {
            self.value.create(f);
        }
        &mut self.value
    }
}

impl<T: Display> Display for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Savable> Savable for Lazy<T> {
    fn save(&self, saver: &mut impl Saver) {
        Deref::deref(self).save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let value = T::load(loader)?;
        Ok(Lazy {
            value: CreateOnce {
                value: UnsafeCell::new(Some(value)),
                once: Once::new(),
                init_called: AtomicBool::new(true),
            },
            init: Mutex::new(None),
        })
    }
}

unsafe impl<T: Send> Send for Lazy<T> {}
unsafe impl<T: Sync> Sync for Lazy<T> {}
impl<T> RefUnwindSafe for Lazy<T> {}

pub struct LazyInitOnce<T> {
    value: CreateOnce<InitOnce<T>>,
    init: Mutex<Option<fn() -> T>>,
}

impl<T> LazyInitOnce<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(f)),
        }
    }

    pub fn created(&self) -> bool {
        self.value.created()
    }

    pub fn initialized(&self) -> bool {
        self.value.initialized()
    }

    pub fn init<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut init = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(init) = init.take() {
            self.value.create(|| InitOnce::new(init()));
        }
        self.value.init(f);
    }

    pub fn safe_init<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>>
    where
        F: FnOnce(&mut T) + UnwindSafe,
    {
        let mut init = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(init) = init.take() {
            self.value.create(|| InitOnce::new(init()));
        }
        self.value.safe_init(f)
    }

    pub fn try_init<F>(&self, f: F) -> Result<(), AlreadyInitialized>
    where
        F: FnOnce(&mut T),
    {
        let mut init = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(init) = init.take() {
            self.value.create(|| InitOnce::new(init()));
        }
        self.value.try_init(f)
    }

    pub fn try_safe_init<F>(&self, f: F) -> Result<(), InitError>
    where
        F: FnOnce(&mut T) + UnwindSafe,
    {
        let mut init = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(init) = init.take() {
            self.value.create(|| InitOnce::new(init()));
        }
        self.value.try_safe_init(f)
    }
}

impl<T: Default> LazyInitOnce<T> {
    pub const fn default() -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(T::default)),
        }
    }
}

impl<T> Deref for LazyInitOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self
            .init
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_some()
        {
            panic!("InitOnce::deref called before InitOnce::init");
        }
        &self.value
    }
}

impl<T: Display> Display for LazyInitOnce<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Savable> Savable for LazyInitOnce<T> {
    fn save(&self, saver: &mut impl Saver) {
        Deref::deref(self).save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let value = T::load(loader)?;
        Ok(LazyInitOnce {
            value: CreateOnce {
                value: UnsafeCell::new(Some(InitOnce {
                    value: UnsafeCell::new(value),
                    once: Once::new(),
                    init_called: AtomicBool::new(true),
                })),
                once: Once::new(),
                init_called: AtomicBool::new(true),
            },
            init: Mutex::new(None),
        })
    }
}

unsafe impl<T: Send> Send for LazyInitOnce<T> {}
unsafe impl<T: Sync> Sync for LazyInitOnce<T> {}
impl<T> RefUnwindSafe for LazyInitOnce<T> {}

#[macro_export]
macro_rules! lazy_init {
    {
        $(
            $v:vis static $n:ident $($k:ident)?: $t:ty = $init:expr;
        )*
    } => {
        $(
            $v static $n $($k)?: $crate::once::LazyInitOnce<$t> = $crate::once::LazyInitOnce::new(|| { $init });
        )*
    };
    {
        $(
            let $n:ident $($k:ident)?$(: $t:ty)? = $init:expr;
        )*
    } => {
        $(
            let $n $($k)?$(: $crate::once::LazyInitOnce<$t>)? = $crate::once::LazyInitOnce::new(|| { $init });
        )*
    };
}

#[macro_export]
macro_rules! lazy {
    {
        $(
            $v:vis static $n:ident $($k:ident)?: $t:ty = $init:expr;
        )*
    } => {
        $(
            $v static $n $($k)?: $crate::once::Lazy<$t> = $crate::once::Lazy::new(|| { $init });
        )*
    };
    {
        $(
            let $n:ident $($k:ident)?$(: $t:ty)? = $init:expr;
        )*
    } => {
        $(
            let $n $($k)?$(: $crate::once::Lazy<$t>)? = $crate::once::Lazy::new(|| { $init });
        )*
    };
}

#[macro_export]
macro_rules! create_once {
     {
        $(
            $v:vis static $n:ident $($k:ident)?: $t:ty;
        )*
    } => {
        $(
            $v static $n $($k)?: $crate::once::CreateOnce<$t> = $crate::once::CreateOnce::new();
        )*
    };
    {
        $(
            let $n:ident $($k:ident)?: $t:ty;
        )*
    } => {
        $(
            let $n $($k)?: $crate::once::CreateOnce<$t> = $crate::once::CreateOnce::new();
        )*
    };
}
