use spa::PodStruct;

use crate::PwMethod;

/// Core::Hello (Opcode 1)
///
/// The first message the client sends
#[derive(Debug, PodStruct)]
pub struct Hello {
    pub version: i32,
}

impl Default for Hello {
    fn default() -> Self {
        Self { version: 3 }
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
#[derive(Debug, PodStruct)]
pub struct Sync {
    pub id: i32,
    pub seq: i32,
}

impl PwMethod for Sync {
    fn id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u8 {
        2
    }
}

/// Core::Pong (Opcode 3)
#[derive(Debug, PodStruct)]
pub struct Pong {
    id: i32,
    seq: i32,
}
