use std::fmt::{Debug, Display};
use crate::utils::SplitInto;

pub struct ByteBuffer {
    buffer: Vec<u8>
}

impl ByteBuffer {
    pub fn new() -> ByteBuffer {
        ByteBuffer {
            buffer: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> ByteBuffer {
        ByteBuffer {
            buffer: Vec::with_capacity(capacity)
        }
    }
}

impl SplitInto for ByteBuffer {
    fn split_into(self, n: usize) -> Vec<Self> {
        self.buffer.split_into(n).into_iter().map(ByteBuffer::from).collect::<Vec<_>>()
    }
}

impl From<Vec<u8>> for ByteBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        ByteBuffer {
            buffer
        }
    }
}

impl From<ByteBuffer> for Vec<u8> {
    fn from(buffer: ByteBuffer) -> Self {
        buffer.buffer
    }
}

impl Display for ByteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}