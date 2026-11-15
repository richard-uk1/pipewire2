use bytes::Bytes;

use crate::{PodBuf, PodDecoder, pod::DecodePodError};

pub trait EncodePod {
    /// Write `self` as a Pod into `dst`
    fn encode(&self, dst: &mut PodBuf);
}

impl EncodePod for i32 {
    fn encode(&self, dst: &mut PodBuf) {
        dst.put_int(*self);
    }
}

impl EncodePod for String {
    fn encode(&self, dst: &mut PodBuf) {
        dst.put_string(self);
    }
}

/// Pop `self` off `src`.
pub trait DecodePod: Sized {
    /// Pop `self` off `src`.
    fn decode(src: &mut Bytes) -> Result<Self, DecodePodError>;
}

impl DecodePod for i32 {
    fn decode(src: &mut Bytes) -> Result<Self, DecodePodError> {
        src.decode_int()
    }
}

impl DecodePod for String {
    fn decode(src: &mut Bytes) -> Result<Self, DecodePodError> {
        src.decode_string()
    }
}
