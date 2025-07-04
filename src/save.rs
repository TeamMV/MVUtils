use std::cell::{Cell, UnsafeCell};
use std::hash::Hash;
use bytebuffer::ByteBuffer;
use std::ops::{Bound, Deref, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};
use std::time::{Duration, Instant, SystemTime};
use hashbrown::{HashMap, HashSet};
use parking_lot::{Mutex, RwLock};
use crate::bytebuffer::ByteBufferExtras;
use crate::utils::Recover;

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
    fn pop_to_end(&mut self) -> Option<Vec<u8>>;
    fn pop_to_end_unchecked(&mut self) -> Vec<u8> {
        self.pop_to_end().unwrap()
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
    
    fn peek_bytes(&mut self, amount: usize) -> Option<Vec<u8>>;
    fn peek_bytes_unchecked(&mut self, amount: usize) -> Vec<u8> {
        self.peek_bytes(amount).unwrap()
    }
    fn peek_bool(&mut self) -> Option<bool>;
    fn peek_bool_unchecked(&mut self) -> bool {
        self.peek_bool().unwrap()
    }
    fn peek_u8(&mut self) -> Option<u8>;
    fn peek_u8_unchecked(&mut self) -> u8 {
        self.peek_u8().unwrap()
    }
    fn peek_u16(&mut self) -> Option<u16>;
    fn peek_u16_unchecked(&mut self) -> u16 {
        self.peek_u16().unwrap()
    }
    fn peek_u32(&mut self) -> Option<u32>;
    fn peek_u32_unchecked(&mut self) -> u32 {
        self.peek_u32().unwrap()
    }
    fn peek_u64(&mut self) -> Option<u64>;
    fn peek_u64_unchecked(&mut self) -> u64 {
        self.peek_u64().unwrap()
    }
    fn peek_i8(&mut self) -> Option<i8>;
    fn peek_i8_unchecked(&mut self) -> i8 {
        self.peek_i8().unwrap()
    }
    fn peek_i16(&mut self) -> Option<i16>;
    fn peek_i16_unchecked(&mut self) -> i16 {
        self.peek_i16().unwrap()
    }
    fn peek_i32(&mut self) -> Option<i32>;
    fn peek_i32_unchecked(&mut self) -> i32 {
        self.peek_i32().unwrap()
    }
    fn peek_i64(&mut self) -> Option<i64>;
    fn peek_i64_unchecked(&mut self) -> i64 {
        self.peek_i64().unwrap()
    }
    fn peek_f32(&mut self) -> Option<f32>;
    fn peek_f32_unchecked(&mut self) -> f32 {
        self.peek_f32().unwrap()
    }
    fn peek_f64(&mut self) -> Option<f64>;
    fn peek_f64_unchecked(&mut self) -> f64 {
        self.peek_f64().unwrap()
    }
}

impl Saver for ByteBuffer {
    fn push_bytes(&mut self, bytes: &[u8]) {
        self.write_bytes(bytes);
    }

    fn push_bool(&mut self, bool: bool) {
        self.write_u8(if bool { 1 } else { 0 });
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

    fn pop_to_end(&mut self) -> Option<Vec<u8>> {
        self.read_bytes(self.len() - self.get_rpos()).ok()
    }

    fn pop_bool(&mut self) -> Option<bool> {
        self.read_u8().ok().map(|v| v != 0)
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

    fn peek_bytes(&mut self, amount: usize) -> Option<Vec<u8>> {
        let rpos = self.get_rpos();
        let bytes = self.read_bytes(amount).ok();
        self.set_rpos(rpos);
        bytes
    }

    fn peek_bool(&mut self) -> Option<bool> {
        let rpos = self.get_rpos();
        let result = self.pop_bool();
        self.set_rpos(rpos);
        result
    }

    fn peek_u8(&mut self) -> Option<u8> {
        let rpos = self.get_rpos();
        let result = self.pop_u8();
        self.set_rpos(rpos);
        result
    }

    fn peek_u16(&mut self) -> Option<u16> {
        let rpos = self.get_rpos();
        let result = self.pop_u16();
        self.set_rpos(rpos);
        result
    }

    fn peek_u32(&mut self) -> Option<u32> {
        let rpos = self.get_rpos();
        let result = self.pop_u32();
        self.set_rpos(rpos);
        result
    }

    fn peek_u64(&mut self) -> Option<u64> {
        let rpos = self.get_rpos();
        let result = self.pop_u64();
        self.set_rpos(rpos);
        result
    }

    fn peek_i8(&mut self) -> Option<i8> {
        let rpos = self.get_rpos();
        let result = self.pop_i8();
        self.set_rpos(rpos);
        result
    }

    fn peek_i16(&mut self) -> Option<i16> {
        let rpos = self.get_rpos();
        let result = self.pop_i16();
        self.set_rpos(rpos);
        result
    }

    fn peek_i32(&mut self) -> Option<i32> {
        let rpos = self.get_rpos();
        let result = self.pop_i32();
        self.set_rpos(rpos);
        result
    }

    fn peek_i64(&mut self) -> Option<i64> {
        let rpos = self.get_rpos();
        let result = self.pop_i64();
        self.set_rpos(rpos);
        result
    }

    fn peek_f32(&mut self) -> Option<f32> {
        let rpos = self.get_rpos();
        let result = self.pop_f32();
        self.set_rpos(rpos);
        result
    }

    fn peek_f64(&mut self) -> Option<f64> {
        let rpos = self.get_rpos();
        let result = self.pop_f64();
        self.set_rpos(rpos);
        result
    }
}

pub trait Savable: Sized {
    fn save(&self, saver: &mut impl Saver);
    fn save_consume(self, saver: &mut impl Saver) {
        self.save(saver);
    }
    fn load(loader: &mut impl Loader) -> Result<Self, String>;
}

pub trait Interpret {
    fn interpret<As: Savable>(self) -> Result<As, String>;
}

impl<T> Interpret for T where T: IntoIterator<Item=u8> {
    fn interpret<As: Savable>(self) -> Result<As, String> {
        let vec: Vec<u8> = self.into_iter().collect();
        let mut buffer = ByteBuffer::from_vec_le(vec);
        As::load(&mut buffer)
    }
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

impl Savable for usize {
    fn save(&self, saver: &mut impl Saver) {
        let mut value = *self;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let mut result: usize = 0;
        let mut shift = 0;
        let usize_bits = size_of::<usize>() * 8;

        loop {
            if shift >= usize_bits {
                return Err("Failed to load usize: Incompatible system bitness".to_string());
            }
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as usize) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        Ok(result)
    }
}

impl Savable for isize {
    fn save(&self, saver: &mut impl Saver) {
        let bits = size_of::<isize>() * 8;
        let zigzag = ( (*self as usize) << 1 ) ^ (((*self) >> (bits - 1)) as usize);
        let mut value = zigzag;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let mut result: usize = 0;
        let mut shift = 0;
        let usize_bits = size_of::<usize>() * 8;

        loop {
            if shift >= usize_bits {
                return Err("Failed to load isize: Incompatible system bitness".to_string());
            }
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as usize) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        let value = ((result >> 1) as isize) ^ (-((result & 1) as isize));
        Ok(value)
    }
}

macro_rules! impl_savable_tuple {
    () => {};
    ($first:ident $($rest:ident)*) => {
        impl_savable_tuple!($($rest)*);
        impl<$first: Savable, $($rest: Savable),*> Savable for ($first, $($rest),*) {
            fn save(&self, saver: &mut impl Saver) {
                #[allow(non_snake_case)]
                let ($first, $($rest),*) = self;
                $first.save(saver);
                $( $rest.save(saver); )*
            }

            fn load(loader: &mut impl Loader) -> Result<Self, String> {
                Ok(($first::load(loader)?,$($rest::load(loader)?),*))
            }
        }
    };
}

impl_savable_tuple!(E D C B A Z Y X W V U T);

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

impl<T: Savable, const N: usize> Savable for [T; N] {
    fn save(&self, saver: &mut impl Saver) {
        self.iter().for_each(|t| t.save(saver));
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        core::array::try_from_fn(|_| T::load(loader))
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

impl<T: Savable + Eq + Hash> Savable for std::collections::HashSet<T> {
    fn save(&self, saver: &mut impl Saver) {
        (self.len() as u64).save(saver);
        self.iter().for_each(|t| t.save(saver));
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let len = u64::load(loader)?;
        let mut set = std::collections::HashSet::with_capacity(len as usize);
        for _ in 0..len {
            set.insert(T::load(loader)?);
        }
        Ok(set)
    }
}

impl<T: Savable + Eq + Hash> Savable for HashSet<T> {
    fn save(&self, saver: &mut impl Saver) {
        (self.len() as u64).save(saver);
        self.iter().for_each(|t| t.save(saver));
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let len = u64::load(loader)?;
        let mut set = HashSet::with_capacity(len as usize);
        for _ in 0..len {
            set.insert(T::load(loader)?);
        }
        Ok(set)
    }
}

impl<K: Savable + Eq + Hash, V: Savable> Savable for std::collections::HashMap<K, V> {
    fn save(&self, saver: &mut impl Saver) {
        (self.len() as u64).save(saver);
        self.iter().for_each(|(k, v)| {
            k.save(saver);
            v.save(saver);
        });
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let len = u64::load(loader)?;
        let mut map = std::collections::HashMap::with_capacity(len as usize);

        for _ in 0..len {
            let k = K::load(loader)?;
            let v = V::load(loader)?;
            map.insert(k, v);
        }

        Ok(map)
    }
}

impl<K: Savable + Eq + Hash, V: Savable> Savable for HashMap<K, V> {
    fn save(&self, saver: &mut impl Saver) {
        (self.len() as u64).save(saver);
        self.iter().for_each(|(k, v)| {
            k.save(saver);
            v.save(saver);
        });
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let len = u64::load(loader)?;
        let mut map = HashMap::with_capacity(len as usize);

        for _ in 0..len {
            let k = K::load(loader)?;
            let v = V::load(loader)?;
            map.insert(k, v);
        }

        Ok(map)
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

impl<T: Savable> Savable for Range<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.start.save(saver);
        self.end.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let start = T::load(loader)?;
        let end = T::load(loader)?;
        Ok(Range { start, end })
    }
}

impl<T: Savable> Savable for RangeInclusive<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.start().save(saver);
        self.end().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let start = T::load(loader)?;
        let end = T::load(loader)?;
        Ok(RangeInclusive::new(start, end))
    }
}

impl<T: Savable> Savable for RangeFrom<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.start.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let start = T::load(loader)?;
        Ok(RangeFrom { start })
    }
}

impl<T: Savable> Savable for RangeTo<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.end.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let end = T::load(loader)?;
        Ok(RangeTo { end })
    }
}

impl<T: Savable> Savable for RangeToInclusive<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.end.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let end = T::load(loader)?;
        Ok(RangeToInclusive { end })
    }
}

impl<T: Savable> Savable for std::sync::RwLock<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.read().recover().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(std::sync::RwLock::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for RwLock<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.read().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(RwLock::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for std::sync::Mutex<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.lock().recover().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(std::sync::Mutex::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for Mutex<T> {
    fn save(&self, saver: &mut impl Saver) {
        self.lock().save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(Mutex::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for UnsafeCell<T> {
    fn save(&self, saver: &mut impl Saver) {
        unsafe { self.get().as_ref().unwrap() }.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(UnsafeCell::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for Cell<T> {
    fn save(&self, saver: &mut impl Saver) {
        unsafe { self.as_ptr().as_ref().unwrap() }.save(saver);
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(Cell::new(T::load(loader)?))
    }
}

impl<T: Savable> Savable for Bound<T> {
    fn save(&self, saver: &mut impl Saver) {
        match self {
            Bound::Included(t) => {
                saver.push_u8(0);
                t.save(saver);
            }
            Bound::Excluded(t) => {
                saver.push_u8(1);
                t.save(saver);
            }
            Bound::Unbounded => saver.push_u8(2),
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let variant = u8::load(loader)?;
        match variant {
            0 => Ok(Bound::Included(T::load(loader)?)),
            1 => Ok(Bound::Excluded(T::load(loader)?)),
            2 => Ok(Bound::Unbounded),
            _ => Err("Failed to load Bound from Loader!".to_string()),
        }
    }
}

#[cfg(feature = "save_str")]
impl Savable for &str {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_string(self);
    }

    fn load(_: &mut impl Loader) -> Result<Self, String> {
        Err("&str may not be loaded, this is part of unstable feature 'save_str'".to_string())
    }
}

pub mod custom {
    use crate::save::{Loader, Savable, Saver};

    pub fn string8_save(saver: &mut impl Saver, str: &String) {
        let bytes = str.as_bytes();
        saver.push_u8(bytes.len().min(255) as u8);
        if bytes.len() > 255 {
            saver.push_bytes(&bytes[..255]);
        } else {
            saver.push_bytes(bytes);
        }
    }

    pub fn string8_load(loader: &mut impl Loader) -> Result<String, String> {
        let len = u8::load(loader)?;
        let bytes = loader.pop_bytes(len as usize).ok_or("Failed to load String8 from Loader!")?;
        Ok(String::from_utf8(bytes).map_err(|e| e.to_string())?)
    }

    pub fn string16_save(saver: &mut impl Saver, str: &String) {
        let bytes = str.as_bytes();
        saver.push_u16(bytes.len().min(65535) as u16);
        if bytes.len() > 65535 {
            saver.push_bytes(&bytes[..65535]);
        } else {
            saver.push_bytes(bytes);
        }
    }

    pub fn string16_load(loader: &mut impl Loader) -> Result<String, String> {
        let len = u16::load(loader)?;
        let bytes = loader.pop_bytes(len as usize).ok_or("Failed to load String16 from Loader!")?;
        Ok(String::from_utf8(bytes).map_err(|e| e.to_string())?)
    }

    pub fn string64_save(saver: &mut impl Saver, str: &String) {
        let bytes = str.as_bytes();
        saver.push_u64(bytes.len() as u64);
        saver.push_bytes(bytes);
    }

    pub fn string64_load(loader: &mut impl Loader) -> Result<String, String> {
        let len = u64::load(loader)?;
        let bytes = loader.pop_bytes(len as usize).ok_or("Failed to load String64 from Loader!")?;
        Ok(String::from_utf8(bytes).map_err(|e| e.to_string())?)
    }

    pub fn vec8_save<T: Savable>(saver: &mut impl Saver, vec: &Vec<T>) {
        saver.push_u8(vec.len().min(255) as u8);
        if vec.len() > 255 {
            for i in 0..255 {
                vec[i].save(saver);
            }
        } else {
            for t in vec {
                t.save(saver);
            }
        }
    }

    pub fn vec8_load<T: Savable>(loader: &mut impl Loader) -> Result<Vec<T>, String> {
        let len = u8::load(loader)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::load(loader)?);
        }
        Ok(vec)
    }

    pub fn vec16_save<T: Savable>(saver: &mut impl Saver, vec: &Vec<T>) {
        saver.push_u16(vec.len().min(65535) as u16);
        if vec.len() > 65535 {
            for i in 0..65535 {
                vec[i].save(saver);
            }
        } else {
            for t in vec {
                t.save(saver);
            }
        }
    }

    pub fn vec16_load<T: Savable>(loader: &mut impl Loader) -> Result<Vec<T>, String> {
        let len = u16::load(loader)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::load(loader)?);
        }
        Ok(vec)
    }

    pub fn vec32_save<T: Savable>(saver: &mut impl Saver, vec: &Vec<T>) {
        saver.push_u32(vec.len().min(4294967295) as u32);
        if vec.len() > 4294967295 {
            for i in 0..4294967295 {
                vec[i].save(saver);
            }
        } else {
            for t in vec {
                t.save(saver);
            }
        }
    }

    pub fn vec32_load<T: Savable>(loader: &mut impl Loader) -> Result<Vec<T>, String> {
        let len = u32::load(loader)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::load(loader)?);
        }
        Ok(vec)
    }
    
    pub fn raw_vec_save<T: Savable>(saver: &mut impl Saver, vec: &Vec<T>) {
        for t in vec {
            t.save(saver);
        }
    }
    
    pub fn raw_vec_load<T: Savable>(loader: &mut impl Loader) -> Result<Vec<T>, String> {
        let mut vec = Vec::new();
        while let Ok(t) = T::load(loader) {
            vec.push(t);
        }
        Ok(vec)
    }
    
    pub fn raw_bytes_save(saver: &mut impl Saver, vec: &Vec<u8>) {
        saver.push_bytes(&vec);
    }
    
    pub fn raw_bytes_load(loader: &mut impl Loader) -> Result<Vec<u8>, String> {
        loader.pop_to_end().ok_or("Failed to load bytes from loader!".to_string())
    }
    
    pub fn empty_vec_load<T>(_: &mut impl Loader) -> Result<Vec<T>, String> {
        Ok(Vec::new())
    }
    
    pub fn ignore_save<T>(_: &mut impl Saver, _: &T) {}
    
    pub fn save<T: Savable>(saver: &mut impl Saver, item: &T) {
        item.save(saver);
    }
    
    pub fn load_default<T: Default>(_: &mut impl Loader) -> Result<T, String> {
        Ok(T::default())
    }
    
    pub fn load<T: Savable>(loader: &mut impl Loader) -> Result<T, String> {
        T::load(loader)
    }

    pub fn varint_save(saver: &mut impl Saver, value: &u64) {
        let mut value = *value;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    pub fn varint_load(loader: &mut impl Loader) -> Result<u64, String> {
        let mut result: u64 = 0;
        let mut shift = 0;
        while shift < 64 {
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        Err("Varint larger than 64 bits".to_string())
    }

    pub fn varint_signed_save(saver: &mut impl Saver, value: &i64) {
        let zigzag = ((*value << 1) ^ (*value >> 63)) as u64;
        let mut value = zigzag;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }
    
    pub fn varint_signed_load(loader: &mut impl Loader) -> Result<i64, String> {
        let mut result: u64 = 0;
        let mut shift = 0;
        while shift < 64 {
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                let value = ((result >> 1) as i64) ^ (-((result & 1) as i64));
                return Ok(value);
            }
            shift += 7;
        }
        Err("Varint larger than 64 bits".to_string())
    }
}

