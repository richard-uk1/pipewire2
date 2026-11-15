use bytes::Bytes;

use crate::{PodBuf, pod::DecodePodError};

pub trait EncodePod {
    /// Write `self` as a Pod into `dst`
    fn encode(&self, dst: &mut PodBuf);
}

pub trait DecodePod: Sized {
    fn decode(src: Bytes) -> Result<Self, DecodePodError>;
}
