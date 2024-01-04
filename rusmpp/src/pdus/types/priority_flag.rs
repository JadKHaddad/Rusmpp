use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

/// Helper for creating a [`PriorityFlag`]
///
/// A priority flag type can not be created from u8.
/// Depending on the variant you want to create, you need to use the variant itself.
///
/// ```rust
/// use rusmpp::pdus::types::priority_flag::GsmSms;
/// use rusmpp::pdus::types::priority_flag::PriorityFlagType;
///
/// let gsm_sms = GsmSms::from(1);
/// assert_eq!(gsm_sms, GsmSms::Priority1);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PriorityFlagType {
    GsmSms(GsmSms),
    GsmCbs(GsmCbs),
    Ansi136(Ansi136),
    Is95(Is95),
    Ansi41Cbs(Ansi41Cbs),
}

impl From<PriorityFlagType> for u8 {
    fn from(value: PriorityFlagType) -> Self {
        match value {
            PriorityFlagType::GsmSms(v) => v.into(),
            PriorityFlagType::GsmCbs(v) => v.into(),
            PriorityFlagType::Ansi136(v) => v.into(),
            PriorityFlagType::Is95(v) => v.into(),
            PriorityFlagType::Ansi41Cbs(v) => v.into(),
        }
    }
}

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoRead,
)]
pub struct PriorityFlag {
    pub value: u8,
}

impl PriorityFlag {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

impl From<PriorityFlagType> for PriorityFlag {
    fn from(value: PriorityFlagType) -> Self {
        Self {
            value: value.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gsm_sms() {
        let gsm_sms = GsmSms::from(0);
        assert_eq!(gsm_sms, GsmSms::None);

        let gsm_sms = GsmSms::from(1);
        assert_eq!(gsm_sms, GsmSms::Priority1);

        let gsm_sms = GsmSms::from(2);
        assert_eq!(gsm_sms, GsmSms::Priority2);

        let gsm_sms = GsmSms::from(3);
        assert_eq!(gsm_sms, GsmSms::Priority3);

        let gsm_sms = GsmSms::from(4);
        assert_eq!(gsm_sms, GsmSms::None);
    }
}
