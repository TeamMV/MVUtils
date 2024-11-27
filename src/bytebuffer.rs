use bytebuffer::{ByteBuffer, Endian};

pub trait ByteBufferExtras: Sized {
    fn new_le() -> Self;
    fn new_be() -> Self;
    fn new_ne() -> Self;

    fn from_vec_le(data: Vec<u8>) -> Self;
    fn from_vec_be(data: Vec<u8>) -> Self;
    fn from_vec_ne(data: Vec<u8>) -> Self;
}

impl ByteBufferExtras for ByteBuffer {
    fn new_le() -> Self {
        let mut buf = ByteBuffer::new();
        buf.set_endian(Endian::LittleEndian);
        buf
    }

    fn new_be() -> Self {
        let mut buf = ByteBuffer::new();
        buf.set_endian(Endian::BigEndian);
        buf
    }

    fn new_ne() -> Self {
        if cfg!(target_endian = "big") {
            Self::new_be()
        } else {
            Self::new_le()
        }
    }

    fn from_vec_le(data: Vec<u8>) -> Self {
        let mut buf = ByteBuffer::from_vec(data);
        buf.set_endian(Endian::LittleEndian);
        buf
    }

    fn from_vec_be(data: Vec<u8>) -> Self {
        let mut buf = ByteBuffer::from_vec(data);
        buf.set_endian(Endian::BigEndian);
        buf
    }

    fn from_vec_ne(data: Vec<u8>) -> Self {
        if cfg!(target_endian = "big") {
            Self::from_vec_be(data)
        } else {
            Self::from_vec_le(data)
        }
    }
}