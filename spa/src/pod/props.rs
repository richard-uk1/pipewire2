use std::fmt;

use indexmap::IndexMap;

use crate::{DecodePod, DecodePodError, EncodePod, PodDecoder};

/// A common pattern in pipewire is a variadic-length struct, where the first field is
/// the number of pairs of strings, and then the rest of the fields are the key/value
/// pairs.
pub struct Props {
    pub inner: IndexMap<String, String>,
}

impl fmt::Debug for Props {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl DecodePod for Props {
    fn decode(src: &mut bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let size = i32::decode(src)? as u32 as usize;
            let mut inner = IndexMap::with_capacity(size);
            for _ in 0..size {
                let key = String::decode(src)?;
                let value = String::decode(src)?;
                inner.insert(key, value);
            }
            Ok(Self { inner })
        })
    }
}

impl EncodePod for Props {
    fn encode(&self, dst: &mut super::PodBuf) {
        dst.put_struct(|dst| {
            dst.put_int(u32::try_from(self.inner.len()).expect("int overflow") as i32);
            for (key, value) in self.inner.iter() {
                dst.put_string(key);
                dst.put_string(value);
            }
        });
    }
}
