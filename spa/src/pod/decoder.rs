use std::{mem, string::FromUtf8Error};

use bytes::{Buf, Bytes, TryGetError};

use crate::pod::{PodType, pad_to_8};

#[derive(Debug)]
pub enum DecodePodError {
    /// Not enough data in the buffer to decode
    NotEnoughData,
    /// Found a pod of incorrect type
    IncorrectType {
        expected: PodType,
        actual: PodType,
    },
    /// There were some bytes left over
    TrailingBytes,
    /// A string didn't finish with b'\0'
    NotNullTerminatedString,
    NonUtf8String {
        source: FromUtf8Error,
    },
}

impl From<TryGetError> for DecodePodError {
    fn from(_value: TryGetError) -> Self {
        Self::NotEnoughData
    }
}

impl From<FromUtf8Error> for DecodePodError {
    fn from(value: FromUtf8Error) -> Self {
        Self::NonUtf8String { source: value }
    }
}

mod sealed {
    use super::{DecodePodError, PodType};

    /// Seals the trait and provides some useful private functions
    pub trait Sealed {
        fn try_get_pod_type(&mut self) -> Result<PodType, DecodePodError>;
    }
}
use sealed::Sealed;

/// Extension trait for [`Bytes`]
pub trait PodDecoder: Sealed {
    fn decode_struct<T>(
        &mut self,
        struct_decoder_fn: impl FnOnce(&mut Self) -> Result<T, DecodePodError>,
    ) -> Result<T, DecodePodError>;

    /// Check the next Pod is an `i32` and return it.
    fn decode_int(&mut self) -> Result<i32, DecodePodError>;

    /// Check the next Pod is an `i64` and return it.
    fn decode_long(&mut self) -> Result<i64, DecodePodError>;

    /// Check the next Pod is a `String` and return it.
    fn decode_string(&mut self) -> Result<String, DecodePodError>;

    /// Get the next pod in raw form.
    fn try_get_next_pod(&mut self) -> Result<RawPod, DecodePodError>;
}

// MARK: impl

impl sealed::Sealed for Bytes {
    fn try_get_pod_type(&mut self) -> Result<PodType, DecodePodError> {
        Ok(PodType::from(
            self.try_get_u32_ne()
                .map_err(|_| DecodePodError::NotEnoughData)?,
        ))
    }
}

impl PodDecoder for Bytes {
    fn decode_struct<T>(
        &mut self,
        struct_decoder_fn: impl FnOnce(&mut Self) -> Result<T, DecodePodError>,
    ) -> Result<T, DecodePodError> {
        let mut raw = self.try_get_next_pod()?;
        raw.check_type(PodType::Struct)?;

        let output = struct_decoder_fn(&mut raw.data)?;
        raw.finish()?;
        Ok(output)
    }

    fn decode_int(&mut self) -> Result<i32, DecodePodError> {
        let mut raw = self.try_get_next_pod()?;
        raw.check_type(PodType::Int)?;

        let value = raw.data.try_get_i32_ne()?;
        raw.finish()?;
        Ok(value)
    }

    fn decode_long(&mut self) -> Result<i64, DecodePodError> {
        let mut raw = self.try_get_next_pod()?;
        raw.check_type(PodType::Long)?;

        let value = raw.data.try_get_i64_ne()?;
        raw.finish()?;
        Ok(value)
    }

    fn decode_string(&mut self) -> Result<String, DecodePodError> {
        let mut raw = self.try_get_next_pod()?;
        raw.check_type(PodType::String)?;

        // Last byte is a null byte, and the rest the string
        if raw.data.last() != Some(&0) {
            return Err(DecodePodError::NotNullTerminatedString);
        }

        // Skip the null byte and utf-8 decode
        raw.data.truncate(raw.data.len() - 1);
        Ok(String::from_utf8(raw.data.to_vec())?)
    }

    fn try_get_next_pod(&mut self) -> Result<RawPod, DecodePodError> {
        let size = self.try_get_u32_ne()? as usize;
        let pod_type = self.try_get_pod_type()?;

        let padding = pad_to_8(size);
        let size_with_padding = size + padding;

        if self.len() < size_with_padding {
            return Err(DecodePodError::NotEnoughData);
        }

        // `data` actually contains the remainder of `self` until we swap it with `self`.
        let mut data = self.split_off(size_with_padding);
        mem::swap(&mut data, self);

        // discard padding
        data.truncate(size);
        Ok(RawPod { pod_type, data })
    }
}

pub struct RawPod {
    pub pod_type: PodType,
    pub data: Bytes,
}

impl RawPod {
    fn check_type(&self, expected: PodType) -> Result<(), DecodePodError> {
        if self.pod_type == expected {
            Ok(())
        } else {
            Err(DecodePodError::IncorrectType {
                expected,
                actual: self.pod_type,
            })
        }
    }

    fn finish(&self) -> Result<(), DecodePodError> {
        if self.data.is_empty() {
            Ok(())
        } else {
            Err(DecodePodError::TrailingBytes)
        }
    }
}
