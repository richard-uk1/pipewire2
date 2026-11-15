use std::collections::BTreeMap;

use bytes::Bytes;
use spa::{DecodePod, DecodePodError, EncodePod, PodBuf, PodDecoder};

use crate::{PwMethod, codec::PwFrame};

mod event;
pub use event::*;

mod method;
pub use method::*;

#[derive(Debug)]
pub enum DecodeError {
    DecodePodError(DecodePodError),
    UnrecognisedOpCode(u8),
}

impl From<DecodePodError> for DecodeError {
    fn from(value: DecodePodError) -> Self {
        Self::DecodePodError(value)
    }
}
