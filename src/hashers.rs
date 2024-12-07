use std::hash::{BuildHasher, Hasher};
use std::mem;

#[derive(Default)]
#[repr(transparent)]
pub struct U64IdentityHasher {
    value: u64,
}

impl Hasher for U64IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.value
    }

    #[inline(always)]
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.value = i
    }
}

impl BuildHasher for U64IdentityHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct U32IdentityHasher {
    value: u32,
}

impl Hasher for U32IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.value as u64
    }

    #[inline(always)]
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.value = i;
    }
}

impl BuildHasher for U32IdentityHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct I64IdentityHasher {
    value: i64,
}

impl Hasher for I64IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.value as u64
    }

    #[inline(always)]
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.value = i;
    }
}

impl BuildHasher for I64IdentityHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct I32IdentityHasher {
    value: i32,
}

impl Hasher for I32IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.value as u64
    }

    #[inline(always)]
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        self.value = i;
    }
}

impl BuildHasher for I32IdentityHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

pub struct UsizeIdentityHasher {
    value: usize,
}

impl Hasher for UsizeIdentityHasher {
    fn finish(&self) -> u64 {
        todo!()
    }

    fn write(&mut self, bytes: &[u8]) {
        unreachable!()
    }

    fn write_usize(&mut self, u: usize) {
        unsafe {
            #[cfg(target_pointer_width = "64")] {
                self.write_u64(mem::transmute::<usize, u64>(u));
            }
            #[cfg(target_pointer_width = "32")] {
                self.write_u64(mem::transmute::<usize, u32>(u));
            }
        }
    }
}