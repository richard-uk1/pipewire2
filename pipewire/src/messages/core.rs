use std::collections::BTreeMap;

use bytes::Bytes;
use spa::{DecodePod, DecodePodError, EncodePod, PodBuf, PodDecoder};

use crate::{PwMethod, codec::PwFrame};

// --- MARK: Methods

/// Core::Hello (Opcode 1)
///
/// The first message the client sends
#[derive(Debug)]
pub struct Hello {
    pub version: i32,
}

impl Default for Hello {
    fn default() -> Self {
        Self { version: 3 }
    }
}

impl EncodePod for Hello {
    fn encode(&self, dst: &mut PodBuf) {
        dst.put_struct(|dst| {
            dst.put_int(self.version);
        });
    }
}

impl PwMethod for Hello {
    fn id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u8 {
        1
    }
}

/// Core::Sync (Opcode 2)
#[derive(Debug)]
pub struct Sync {
    pub id: i32,
    pub seq: i32,
}

impl EncodePod for Sync {
    fn encode(&self, dst: &mut PodBuf) {
        dst.put_struct(|dst| {
            dst.put_int(self.id);
            dst.put_int(self.seq);
        });
    }
}

impl PwMethod for Sync {
    fn id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u8 {
        2
    }
}

// --- MARK: Events

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

/// Statically typed event covering all possible core events
#[derive(Debug)]
pub enum Event {
    Info(Info),
    Done(Done),
    Ping(Ping),
    Error(Error),
    RemoveId(RemoveId),
    BoundId(BoundId),
    // AddMem(AddMem),
    // RemoveMem(RemoveMem),
    // BoundProps(BoundProps),
}

impl From<Info> for Event {
    fn from(value: Info) -> Self {
        Self::Info(value)
    }
}

impl From<Done> for Event {
    fn from(value: Done) -> Self {
        Self::Done(value)
    }
}

impl From<Ping> for Event {
    fn from(value: Ping) -> Self {
        Self::Ping(value)
    }
}

impl From<Error> for Event {
    fn from(value: Error) -> Self {
        Self::Error(value)
    }
}

impl From<RemoveId> for Event {
    fn from(value: RemoveId) -> Self {
        Self::RemoveId(value)
    }
}

impl From<BoundId> for Event {
    fn from(value: BoundId) -> Self {
        Self::BoundId(value)
    }
}

impl Event {
    pub fn decode(frame: PwFrame) -> Result<Self, DecodeError> {
        let data = frame.payload;
        Ok(match frame.opcode {
            0 => Info::decode(data)?.into(),
            1 => Done::decode(data)?.into(),
            2 => Ping::decode(data)?.into(),
            3 => Error::decode(data)?.into(),
            4 => RemoveId::decode(data)?.into(),
            5 => BoundId::decode(data)?.into(),
            other => return Err(DecodeError::UnrecognisedOpCode(other)),
        })
    }
}

#[derive(Debug)]
pub struct Info {
    id: i32,
    cookie: i32,
    user_name: String,
    host_name: String,
    version: String,
    name: String,
    change_mask: i64,
    props: BTreeMap<String, String>,
}

impl DecodePod for Info {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            let cookie = src.decode_int()?;
            let user_name = src.decode_string()?;
            let host_name = src.decode_string()?;
            let version = src.decode_string()?;
            let name = src.decode_string()?;
            let change_mask = src.decode_long()?;
            let props = src.decode_struct(|src| {
                let n_items = src.decode_int()?;
                (0..n_items)
                    .map(|_| Ok((src.decode_string()?, src.decode_string()?)))
                    .collect::<Result<_, DecodePodError>>()
            })?;
            Ok(Self {
                id,
                cookie,
                user_name,
                host_name,
                version,
                name,
                change_mask,
                props,
            })
        })
    }
}

#[derive(Debug)]
pub struct Done {
    id: i32,
    seq: i32,
}

impl DecodePod for Done {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            let seq = src.decode_int()?;
            Ok(Self { id, seq })
        })
    }
}

#[derive(Debug)]
pub struct Ping {
    id: i32,
    seq: i32,
}

impl DecodePod for Ping {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            let seq = src.decode_int()?;
            Ok(Self { id, seq })
        })
    }
}

#[derive(Debug)]
pub struct Error {
    id: i32,
    seq: i32,
    res: i32,
    message: String,
}

impl DecodePod for Error {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            let seq = src.decode_int()?;
            let res = src.decode_int()?;
            let message = src.decode_string()?;
            Ok(Self {
                id,
                seq,
                res,
                message,
            })
        })
    }
}

#[derive(Debug)]
pub struct RemoveId {
    id: i32,
}

impl DecodePod for RemoveId {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            Ok(Self { id })
        })
    }
}

#[derive(Debug)]
pub struct BoundId {
    id: i32,
    global_id: i32,
}

impl DecodePod for BoundId {
    fn decode(mut src: bytes::Bytes) -> Result<Self, DecodePodError> {
        src.decode_struct(|src| {
            let id = src.decode_int()?;
            let global_id = src.decode_int()?;
            Ok(Self { id, global_id })
        })
    }
}
