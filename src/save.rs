use bytebuffer::ByteBuffer;
use std::ops::Deref;
use std::time::{Duration, Instant, SystemTime};

pub trait Saver {
    fn push_bytes(&mut self, bytes: &[u8]);
    fn push_bool(&mut self, bool: bool);
    fn push_u8(&mut self, value: u8);
    fn push_u16(&mut self, value: u16);
    fn push_u32(&mut self, value: u32);
    fn push_u64(&mut self, value: u64);
    fn push_i8(&mut self, value: i8);
    fn push_i16(&mut self, value: i16);
    fn push_i32(&mut self, value: i32);
    fn push_i64(&mut self, value: i64);
    fn push_f32(&mut self, value: f32);
    fn push_f64(&mut self, value: f64);
    fn push_string(&mut self, value: &str);
}

pub trait Loader {
    fn pop_bytes(&mut self, amount: usize) -> Option<Vec<u8>>;
    fn pop_bytes_unchecked(&mut self, amount: usize) -> Vec<u8> {
        self.pop_bytes(amount).unwrap()
    }
    fn pop_bool(&mut self) -> Option<bool>;
    fn pop_bool_unchecked(&mut self) -> bool {
        self.pop_bool().unwrap()
    }
    fn pop_u8(&mut self) -> Option<u8>;
    fn pop_u8_unchecked(&mut self) -> u8 {
        self.pop_u8().unwrap()
    }
    fn pop_u16(&mut self) -> Option<u16>;
    fn pop_u16_unchecked(&mut self) -> u16 {
        self.pop_u16().unwrap()
    }
    fn pop_u32(&mut self) -> Option<u32>;
    fn pop_u32_unchecked(&mut self) -> u32 {
        self.pop_u32().unwrap()
    }
    fn pop_u64(&mut self) -> Option<u64>;
    fn pop_u64_unchecked(&mut self) -> u64 {
        self.pop_u64().unwrap()
    }
    fn pop_i8(&mut self) -> Option<i8>;
    fn pop_i8_unchecked(&mut self) -> i8 {
        self.pop_i8().unwrap()
    }
    fn pop_i16(&mut self) -> Option<i16>;
    fn pop_i16_unchecked(&mut self) -> i16 {
        self.pop_i16().unwrap()
    }
    fn pop_i32(&mut self) -> Option<i32>;
    fn pop_i32_unchecked(&mut self) -> i32 {
        self.pop_i32().unwrap()
    }
    fn pop_i64(&mut self) -> Option<i64>;
    fn pop_i64_unchecked(&mut self) -> i64 {
        self.pop_i64().unwrap()
    }
    fn pop_f32(&mut self) -> Option<f32>;
    fn pop_f32_unchecked(&mut self) -> f32 {
        self.pop_f32().unwrap()
    }
    fn pop_f64(&mut self) -> Option<f64>;
    fn pop_f64_unchecked(&mut self) -> f64 {
        self.pop_f64().unwrap()
    }
    fn pop_string(&mut self) -> Option<String>;
    fn pop_string_unchecked(&mut self) -> String {
        self.pop_string().unwrap()
    }
}

impl Saver for ByteBuffer {
    fn push_bytes(&mut self, bytes: &[u8]) {
        self.write_bytes(bytes);
    }

    fn push_bool(&mut self, bool: bool) {
        self.write_bit(bool);
    }

    fn push_u8(&mut self, value: u8) {
        self.write_u8(value);
    }

    fn push_u16(&mut self, value: u16) {
        self.write_u16(value);
    }

    fn push_u32(&mut self, value: u32) {
        self.write_u32(value);
    }

    fn push_u64(&mut self, value: u64) {
        self.write_u64(value);
    }

    fn push_i8(&mut self, value: i8) {
        self.write_i8(value);
    }

    fn push_i16(&mut self, value: i16) {
        self.write_i16(value);
    }

    fn push_i32(&mut self, value: i32) {
        self.write_i32(value);
    }

    fn push_i64(&mut self, value: i64) {
        self.write_i64(value);
    }

    fn push_f32(&mut self, value: f32) {
        self.write_f32(value);
    }

    fn push_f64(&mut self, value: f64) {
        self.write_f64(value);
    }

    fn push_string(&mut self, value: &str) {
        self.write_string(value);
    }
}

impl Loader for ByteBuffer {
    fn pop_bytes(&mut self, amount: usize) -> Option<Vec<u8>> {
        self.read_bytes(amount).ok()
    }

    fn pop_bool(&mut self) -> Option<bool> {
        self.read_bit().ok()
    }

    fn pop_u8(&mut self) -> Option<u8> {
        self.read_u8().ok()
    }

    fn pop_u16(&mut self) -> Option<u16> {
        self.read_u16().ok()
    }

    fn pop_u32(&mut self) -> Option<u32> {
        self.read_u32().ok()
    }

    fn pop_u64(&mut self) -> Option<u64> {
        self.read_u64().ok()
    }

    fn pop_i8(&mut self) -> Option<i8> {
        self.read_i8().ok()
    }

    fn pop_i16(&mut self) -> Option<i16> {
        self.read_i16().ok()
    }

    fn pop_i32(&mut self) -> Option<i32> {
        self.read_i32().ok()
    }

    fn pop_i64(&mut self) -> Option<i64> {
        self.read_i64().ok()
    }

    fn pop_f32(&mut self) -> Option<f32> {
        self.read_f32().ok()
    }

    fn pop_f64(&mut self) -> Option<f64> {
        self.read_f64().ok()
    }

    fn pop_string(&mut self) -> Option<String> {
        self.read_string().ok()
    }
}

pub trait Savable: Sized {
    fn save(&self, saver: &mut impl Saver);
    fn load(loader: &mut impl Loader) -> Result<Self, String>;
}

macro_rules! impl_savable_primitive {
    ($($t:ty, $pu:ident, $po:ident),*) => {
        $(
            impl Savable for $t {
                fn save(&self, saver: &mut impl Saver) {
                    saver.$pu(*self)
                }

                fn load(loader: &mut impl Loader) -> Result<Self, String> {
                    loader.$po().ok_or(format!("Failed to load {} from Loader!", stringify!($t)))
                }
            }
        )*
    };
}

impl_savable_primitive!(
    bool, push_bool, pop_bool, u8, push_u8, pop_u8, u16, push_u16, pop_u16, u32, push_u32, pop_u32,
    u64, push_u64, pop_u64, i8, push_i8, pop_i8, i16, push_i16, pop_i16, i32, push_i32, pop_i32,
    i64, push_i64, pop_i64, f32, push_f32, pop_f32, f64, push_f64, pop_f64
);

impl Savable for String {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_string(self);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        loader
            .pop_string()
            .ok_or("Failed to load String from Loader!".to_string())
    }
}

impl<T: Savable> Savable for Option<T> {
    fn save(&self, saver: &mut impl Saver) {
        match self {
            None => saver.push_u8(0),
            Some(t) => {
                saver.push_u8(1);
                t.save(saver);
            }
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        match u8::load(loader)? {
            0 => Ok(None),
            1 => Ok(Some(T::load(loader)?)),
            _ => Err("Failed to load Option from Loader!".to_string()),
        }
    }
}

impl<T: Savable, E: Savable> Savable for Result<T, E> {
    fn save(&self, saver: &mut impl Saver) {
        match self {
            Ok(t) => {
                saver.push_u8(0);
                t.save(saver);
            }
            Err(e) => {
                saver.push_u8(1);
                e.save(saver);
            }
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        match u8::load(loader)? {
            0 => Ok(Ok(T::load(loader)?)),
            1 => Ok(Err(E::load(loader)?)),
            _ => Err("Failed to load Result from Loader!".to_string()),
        }
    }
}

impl<T: Savable> Savable for Vec<T> {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_u64(self.len() as u64);
        for t in self {
            t.save(saver);
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let len = u64::load(loader)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::load(loader)?);
        }
        Ok(vec)
    }
}

impl Savable for ByteBuffer {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_u64(self.len() as u64);
        saver.push_bytes(self.as_bytes());
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Vec::<u8>::load(loader).map(Into::into)
    }
}

impl<T: Savable> Savable for Box<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.deref().save(saver)
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(Box::new(T::load(loader)?))
    }
}

impl Savable for Duration {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_u64(self.as_secs());
        saver.push_u32(self.subsec_nanos());
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let secs = u64::load(loader)?;
        let nanos = u32::load(loader)?;

        Ok(Duration::new(secs, nanos))
    }
}

impl Savable for Instant {
    fn save(&self, saver: &mut impl Saver) {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default() - Instant::now().duration_since(*self);
        duration.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let duration = Duration::load(loader)?;
        Ok(Instant::now() + duration - SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default())
    }
}

impl Savable for SystemTime {
    fn save(&self, saver: &mut impl Saver) {
        self.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let duration = Duration::load(loader)?;
        Ok(SystemTime::UNIX_EPOCH + duration)
    }
}

mod short_string {
    use std::fmt::Display;
    use std::ops::{Deref, DerefMut};
    use crate::save::{Loader, Savable, Saver};

    /// # Short String
    ///
    /// This struct is identical to [`String`], and dereferences to [`String`] and [`str`]
    ///
    /// The only difference, is when saved with a [`Saver`], the length of a short string will be
    /// saved as a `u8`, not a `u32`
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct ShortString(String);

    impl ShortString {
        pub fn new(s: impl Into<String>) -> Self {
            Self(s.into())
        }

        pub fn extract(self) -> String {
            self.0
        }
    }

    impl Savable for ShortString {
        fn save(&self, saver: &mut impl Saver) {
            saver.push_u8(self.0.len() as u8);
            saver.push_bytes(self.0.as_bytes());
        }

        fn load(loader: &mut impl Loader) -> Result<Self, String> {
            let len = u8::load(loader)?;
            let bytes = loader.pop_bytes(len as usize).ok_or("Failed to load ShortString from Loader!")?;
            Ok(Self(String::from_utf8(bytes).map_err(|e| e.to_string())?))
        }
    }

    impl<T: Into<String>> From<T> for ShortString {
        fn from(value: T) -> Self {
            Self::new(value.into())
        }
    }

    pub trait ToShortString {
        fn to_short_string(&self) -> ShortString;
    }

    impl<T: ToString> ToShortString for T {
        fn to_short_string(&self) -> ShortString {
            self.to_string().into()
        }
    }

    impl Deref for ShortString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for ShortString {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl AsRef<String> for ShortString {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }

    impl AsMut<String> for ShortString {
        fn as_mut(&mut self) -> &mut String {
            &mut self.0
        }
    }

    impl AsRef<str> for ShortString {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Display for ShortString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
}

pub use short_string::*;

