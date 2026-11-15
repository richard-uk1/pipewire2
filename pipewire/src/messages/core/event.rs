use std::collections::BTreeMap;

use spa::{DecodePod, DecodePodError, PodDecoder, PodStruct, Props};

use crate::{codec::PwFrame, messages::core::DecodeError};

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
        let mut data = frame.payload;
        Ok(match frame.opcode {
            0 => Info::decode(&mut data)?.into(),
            1 => Done::decode(&mut data)?.into(),
            2 => Ping::decode(&mut data)?.into(),
            3 => Error::decode(&mut data)?.into(),
            4 => RemoveId::decode(&mut data)?.into(),
            5 => BoundId::decode(&mut data)?.into(),
            other => return Err(DecodeError::UnrecognisedOpCode(other)),
        })
    }
}

#[derive(Debug, PodStruct)]
pub struct Info {
    pub id: i32,
    pub cookie: i32,
    pub user_name: String,
    pub host_name: String,
    pub version: String,
    pub name: String,
    pub change_mask: i64,
    pub props: Props,
}

#[derive(Debug, PodStruct)]
pub struct Done {
    pub id: i32,
    pub seq: i32,
}

#[derive(Debug, PodStruct)]
pub struct Ping {
    pub id: i32,
    pub seq: i32,
}

#[derive(Debug, PodStruct)]
pub struct Error {
    pub id: i32,
    pub seq: i32,
    pub res: i32,
    pub message: String,
}

#[derive(Debug, PodStruct)]
pub struct RemoveId {
    pub id: i32,
}

#[derive(Debug, PodStruct)]
pub struct BoundId {
    pub id: i32,
    pub global_id: i32,
}
