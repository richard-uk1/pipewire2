use std::fmt;

use bytes::{Buf, BufMut, BytesMut};

pub const HEADER_LEN: usize = 16;

#[derive(Clone)]
pub struct Header {
    pub id: u32,
    pub op_size: u32,
    pub seq: u32,
    pub n_fds: u32,
}

impl Header {
    pub fn new(id: u32, opcode: u8, seq: u32, n_fds: u32, size: usize) -> Self {
        let op_size = encode_op_size(opcode, size);
        Self {
            id,
            op_size,
            seq,
            n_fds,
        }
    }

    /// Use this when you plan to fill in the other values later
    pub fn new_placeholder(id: u32, opcode: u8, seq: u32) -> Self {
        Self::new(id, opcode, seq, 0, 0)
    }

    /// Get the opcode from the op_size field
    pub fn opcode(&self) -> u8 {
        (self.op_size >> 24) as u8
    }

    pub fn set_opcode(&mut self, op: u8) {
        self.op_size = encode_op_size(op, self.size());
    }

    /// Get the size from the op_size field
    pub fn size(&self) -> usize {
        (self.op_size & 0xFFFFFF) as usize
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.op_size = encode_op_size(self.opcode(), new_size);
    }

    /// Fills the first 4 bytes with a u32 containing the op_size
    pub fn update_op_size(&self, mut buf: &mut [u8]) {
        buf.put_u32_ne(self.op_size);
    }

    /// Fills the first 4 bytes with a u32 containing n_fds
    pub fn update_n_fds(&self, mut buf: &mut [u8]) {
        buf.put_u32_ne(self.n_fds);
    }

    pub fn parse(mut input: &[u8]) -> Option<Self> {
        let id = input.try_get_u32_ne().ok()?;
        let op_size = input.try_get_u32_ne().ok()?;
        let seq = input.try_get_u32_ne().ok()?;
        let n_fds = input.try_get_u32_ne().ok()?;
        Some(Self {
            id,
            op_size,
            seq,
            n_fds,
        })
    }

    pub fn encode_to(&self, dst: &mut BytesMut) {
        dst.put_u32_ne(self.id);
        dst.put_u32_ne(self.op_size);
        dst.put_u32_ne(self.seq);
        dst.put_u32_ne(self.n_fds);
    }

    pub fn size_inc_header(&self) -> usize {
        self.size() + HEADER_LEN
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("id", &self.id)
            .field("op_size", &self.op_size)
            .field("seq", &self.seq)
            .field("n_fds", &self.n_fds)
            .field("opcode", &self.opcode())
            .field("size", &self.size())
            .finish()
    }
}

/// Combined opcode + size
pub(crate) fn encode_op_size(op: u8, size: usize) -> u32 {
    assert!(size < (1 << 24), "must fit in 3 bytes");
    (size as u32) | (op as u32) << 24
}
