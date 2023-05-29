use std::any::Any;
use std::sync::{Once, atomic::{AtomicBool, Ordering}, Mutex, Arc};
use std::ops::Deref;
use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::panic::{catch_unwind, UnwindSafe};

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
    Panicked(Box<dyn Any + Send +'static>)
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

    pub fn init<F>(&self, f: F) where F: FnOnce(&mut T) {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("InitOnce::init called twice");
        }

        self.once.call_once(|| {
            let value = unsafe { &mut *self.value.get() };
            f(value);
        });
    }

    pub fn safe_init<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>> where F: FnOnce(&mut T) + UnwindSafe {
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
        panicked.lock().unwrap().take().unwrap()
    }

    pub fn try_init<F>(&self, f: F) -> Result<(), AlreadyInitialized> where F: FnOnce(&mut T) {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(AlreadyInitialized);
        }

        self.once.call_once(|| {
            let value = unsafe { &mut *self.value.get() };
            f(value);
        });

        Ok(())
    }

    pub fn try_safe_init<F>(&self, f: F) -> Result<(), InitError> where F: FnOnce(&mut T) + UnwindSafe {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(InitError::AlreadyInitialized(AlreadyInitialized));
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));
        let clone = panicked.clone();

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = unsafe { &mut *self.value.get() };
                f(value);
            });

            if let Err(e) = result {
                clone.lock().unwrap().replace(Err(InitError::Panicked(e)));
            }
        });
        panicked.lock().unwrap().take().unwrap()
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

    pub fn create<F>(&self, f: F) where F: FnOnce() -> T {
        if self.init_called.swap(true, Ordering::SeqCst) {
            panic!("CreateOnce::create called twice");
        }

        self.once.call_once(|| {
            let value = f();
            unsafe { &mut *self.value.get() }.replace(value);
        });
    }

    pub fn safe_create<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>> where F: FnOnce() -> T + UnwindSafe {
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
        panicked.lock().unwrap().take().unwrap()
    }

    pub fn try_create<F>(&self, f: F) -> Result<(), AlreadyInitialized> where F: FnOnce() -> T {
        if self.init_called.swap(true, Ordering::SeqCst) {
            return Err(AlreadyInitialized);
        }

        self.once.call_once(|| {
            let value = f();
            unsafe { &mut *self.value.get() }.replace(value);
        });

        Ok(())
    }

    pub fn try_safe_create<F>(&self, f: F) -> Result<(), InitError> where F: FnOnce() -> T + UnwindSafe {
        if self.init_called.swap(true, Ordering::SeqCst) {
            Err(InitError::AlreadyInitialized(AlreadyInitialized))
        }

        let panicked = Arc::new(Mutex::new(Some(Ok(()))));
        let clone = panicked.clone();

        self.once.call_once(|| {
            let result = catch_unwind(|| {
                let value = f();
                unsafe { &mut *self.value.get() }.replace(value);
            });

            if let Err(e) = result {
                clone.lock().unwrap().replace(Err(InitError::Panicked(e)));
            }
        });
        panicked.lock().unwrap().take().unwrap()
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

        unsafe { self.value.get().as_ref().unwrap() }.as_ref().unwrap()
    }
}

pub struct Lazy<T> {
    value: CreateOnce<T>,
    init: Mutex<Option<fn() -> T>>
}

impl<T> Lazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(f))
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
            init: Mutex::new(Some(T::default))
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

pub struct LazyInitOnce<T> {
    value: CreateOnce<InitOnce<T>>,
    init: Mutex<Option<fn() -> T>>
}

impl<T> LazyInitOnce<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(f))
        }
    }

    pub fn created(&self) -> bool {
        self.value.created()
    }

    pub fn initialized(&self) -> bool {
        self.value.initialized()
    }

    pub fn init<F>(&self, f: F) where F: FnOnce(&mut T) {
        self.value.init(f);
    }

    pub fn safe_init<F>(&self, f: F) -> Result<(), Box<dyn Any + Send + 'static>> where F: FnOnce(&mut T) + UnwindSafe {
        self.value.safe_init(f)
    }

    pub fn try_init<F>(&self, f: F) -> Result<(), AlreadyInitialized> where F: FnOnce(&mut T) {
        self.value.try_init(f)
    }

    pub fn try_safe_init<F>(&self, f: F) -> Result<(), InitError> where F: FnOnce(&mut T) + UnwindSafe {
        self.value.try_safe_init(f)
    }
}

impl<T: Default> LazyInitOnce<T> {
    pub const fn default() -> Self {
        Self {
            value: CreateOnce::new(),
            init: Mutex::new(Some(T::default))
        }
    }
}

impl<T> Deref for LazyInitOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut f = self.init.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(f) = f.take() {
            self.value.create(|| InitOnce::new(f()));
        }
        &self.value
    }
}