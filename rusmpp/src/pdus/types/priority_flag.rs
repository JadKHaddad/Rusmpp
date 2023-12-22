use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

/// The priority_flag parameter allows the originating SME to assign a priority level to the short
/// message
///
/// When priority_flag is deserilized, it will always be Other(u8) variant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PriorityFlag {
    GsmSms(GsmSms),
    GsmCbs(GsmCbs),
    Ansi136(Ansi136),
    Is95(Is95),
    Ansi41Cbs(Ansi41Cbs),
    Other(u8),
}

impl Default for PriorityFlag {
    fn default() -> Self {
        Self::GsmSms(GsmSms::default())
    }
}

impl From<u8> for PriorityFlag {
    fn from(value: u8) -> Self {
        Self::Other(value)
    }
}

impl From<PriorityFlag> for u8 {
    fn from(value: PriorityFlag) -> Self {
        match value {
            PriorityFlag::GsmSms(v) => v.into(),
            PriorityFlag::GsmCbs(v) => v.into(),
            PriorityFlag::Ansi136(v) => v.into(),
            PriorityFlag::Is95(v) => v.into(),
            PriorityFlag::Ansi41Cbs(v) => v.into(),
            PriorityFlag::Other(v) => v,
        }
    }
}

impl IoLength for PriorityFlag {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for PriorityFlag {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for PriorityFlag {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Default,
)]
pub enum GsmSms {
    #[default]
    None = 0,
    Priority1 = 1,
    Priority2 = 2,
    Priority3 = 3,
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Default,
)]
pub enum GsmCbs {
    #[default]
    Noraml = 0,
    ImmediateBroadcast = 1,
    HighPriority = 2,
    Reseverd = 3,
    PriorityBackground = 4,
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Default,
)]
pub enum Ansi136 {
    #[default]
    Buld = 0,
    Noraml = 1,
    Urgent = 2,
    VeryUrgent = 3,
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Default,
)]
pub enum Is95 {
    #[default]
    Noraml = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Default,
)]
pub enum Ansi41Cbs {
    #[default]
    Noraml = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}
