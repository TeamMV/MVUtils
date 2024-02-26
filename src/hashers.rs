use std::hash::{BuildHasher, Hasher};

#[derive(Default)]
pub struct U64IdentityHasher {
    value: u64,
}

impl Hasher for U64IdentityHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

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
pub struct U32IdentityHasher {
    value: u32,
}

impl Hasher for U32IdentityHasher {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

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
pub struct I64IdentityHasher {
    value: i64,
}

impl Hasher for I64IdentityHasher {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

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
pub struct I32IdentityHasher {
    value: i32,
}

impl Hasher for I32IdentityHasher {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

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