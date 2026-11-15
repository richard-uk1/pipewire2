use bytes::Bytes;
use spa::DecodePodError;

use crate::messages::core::{self, DecodeError};

#[derive(Debug, Clone)]
pub struct PwFrame {
    pub id: u32,
    pub opcode: u8,
    pub seq: u32,
    // pub n_fds: u32,
    pub payload: Bytes,
}

impl PwFrame {
    pub fn try_decode_core(self) -> Result<Option<core::Event>, DecodeError> {
        if self.id != 0 {
            return Ok(None);
        }

        core::Event::decode(self).map(Some)
    }
}
