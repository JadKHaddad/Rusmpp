use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

/// The priority_flag parameter allows the originating SME to assign a priority level to the short
/// message
///
/// When priority_flag is deserilized, it will always be Other(u8) variant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoU8)]
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
