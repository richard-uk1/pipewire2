//! NOTE: I'm assuming native endianness throughout
use bytes::BufMut;
pub use spa_macros::Pod;
use std::ffi::CStr;
use thiserror::Error;

mod pod;
pub use pod::{DecodePod, DecodePodError, EncodePod, PodBuf, PodDecoder};

pub mod spa_type;

#[cfg(target_pointer_width = "16")]
compile_error!("this crate assumes that pointers are at least 32 bits wide");

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Overflow {
        #[from]
        source: Overflow,
    },
}

#[derive(Debug, Error)]
#[error("integer overflow")]
pub struct Overflow;

pub trait Pod {
    /// Size of the pod in bytes (not including the 8 header bytes or any required padding)
    fn size(&self) -> Result<u32, Overflow>;

    /// Write `self` to `buf`.
    fn write<B: BufMut>(&self, buf: B) -> Result<(), Error>;
}

trait PodExt: Pod {
    fn write_header<B: BufMut>(&self, mut buf: B, spa_type: u32) -> Result<(), Error> {
        buf.put_u32_ne(self.size()?);
        buf.put_u32_ne(spa_type);
        Ok(())
    }
}

impl<T> PodExt for T where T: Pod {}

/// A special type for an empty value (we don't have a `null` in Rust).
pub struct None;

impl Pod for None {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(0)
    }

    fn write<B: BufMut>(&self, buf: B) -> Result<(), Error> {
        self.write_header(buf, spa_type::NONE)
    }
}

// Bool
impl Pod for bool {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::BOOL)?;
        buf.put_u32_ne(if *self { 1 } else { 0 });
        Ok(())
    }
}

pub struct Id {
    pub value: u32,
}

impl Pod for Id {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::ID)?;
        buf.put_u32_ne(self.value);
        Ok(())
    }
}

// Int
impl Pod for i32 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::INT)?;
        buf.put_i32_ne(*self);
        Ok(())
    }
}

// Sometimes Int is interpreted as u32 so implement for `u32` for
// convenience
impl Pod for u32 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::INT)?;
        buf.put_u32_ne(*self);
        Ok(())
    }
}

// Long
impl Pod for i64 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(8)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::LONG)?;
        buf.put_i64_ne(*self);
        Ok(())
    }
}

// Sometimes Int is interpreted as u32 so implement for `u32` for
// convenience
impl Pod for u64 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(8)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::LONG)?;
        buf.put_u64_ne(*self);
        Ok(())
    }
}

// Float
impl Pod for f32 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::FLOAT)?;
        buf.put_f32_ne(*self);
        Ok(())
    }
}

// Double
impl Pod for f64 {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(4)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::DOUBLE)?;
        buf.put_f64_ne(*self);
        Ok(())
    }
}

// String
impl Pod for &CStr {
    fn size(&self) -> Result<u32, Overflow> {
        self.count_bytes()
            .checked_add(1)
            .ok_or(Overflow)?
            .try_into()
            .ok()
            .ok_or(Overflow)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::STRING)?;
        buf.put_slice(self.to_bytes_with_nul());
        Ok(())
    }
}

// Bytes
impl Pod for &[u8] {
    fn size(&self) -> Result<u32, Overflow> {
        self.len().try_into().ok().ok_or(Overflow)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::STRING)?;
        buf.put_slice(self);
        Ok(())
    }
}

// Rectangle
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Pod for Rectangle {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(8)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::RECTANGLE)?;
        buf.put_u32_ne(self.width);
        buf.put_u32_ne(self.height);
        Ok(())
    }
}

// Fraction
pub struct Fraction {
    pub num: u32,
    pub denom: u32,
}

impl Pod for Fraction {
    fn size(&self) -> Result<u32, Overflow> {
        Ok(8)
    }

    fn write<B: BufMut>(&self, mut buf: B) -> Result<(), Error> {
        self.write_header(&mut buf, spa_type::FRACTION)?;
        buf.put_u32_ne(self.num);
        buf.put_u32_ne(self.denom);
        Ok(())
    }
}

// skip bitmap

// Array
