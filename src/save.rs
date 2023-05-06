use bytebuffer::ByteBuffer;

pub trait Saver {
    fn push_bytes(&mut self, bytes: &[u8]);
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
    fn serialize(&self, serializer: &mut impl Saver);
    fn deserialize(deserializer: &mut impl Loader) -> Result<Self, String>;
}