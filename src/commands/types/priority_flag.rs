/// Helper for creating a priority_flag.
///
/// A priority flag type can not be created from u8.
/// Depending on the variant you want to create, you need to use the variant itself.
///
/// # Example
///
/// ```rust
/// use rusmpp::commands::types::priority_flag::{GsmSms, PriorityFlagType};
///
/// let gsm_sms = GsmSms::from(1);
/// assert_eq!(gsm_sms, GsmSms::Priority1);
/// let priority_flag: u8 = gsm_sms.into();
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
            PriorityFlagType::GsmSms(value) => value.into(),
            PriorityFlagType::GsmCbs(value) => value.into(),
            PriorityFlagType::Ansi136(value) => value.into(),
            PriorityFlagType::Is95(value) => value.into(),
            PriorityFlagType::Ansi41Cbs(value) => value.into(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GsmSms {
    #[default]
    None = 0,
    Priority1 = 1,
    Priority2 = 2,
    Priority3 = 3,
}

impl From<u8> for GsmSms {
    fn from(value: u8) -> Self {
        match value {
            0 => GsmSms::None,
            1 => GsmSms::Priority1,
            2 => GsmSms::Priority2,
            3 => GsmSms::Priority3,
            _ => GsmSms::None,
        }
    }
}

impl From<GsmSms> for u8 {
    fn from(value: GsmSms) -> Self {
        match value {
            GsmSms::None => 0,
            GsmSms::Priority1 => 1,
            GsmSms::Priority2 => 2,
            GsmSms::Priority3 => 3,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GsmCbs {
    #[default]
    Noraml = 0,
    ImmediateBroadcast = 1,
    HighPriority = 2,
    Reseverd = 3,
    PriorityBackground = 4,
}

impl From<u8> for GsmCbs {
    fn from(value: u8) -> Self {
        match value {
            0 => GsmCbs::Noraml,
            1 => GsmCbs::ImmediateBroadcast,
            2 => GsmCbs::HighPriority,
            3 => GsmCbs::Reseverd,
            4 => GsmCbs::PriorityBackground,
            _ => GsmCbs::Noraml,
        }
    }
}

impl From<GsmCbs> for u8 {
    fn from(value: GsmCbs) -> Self {
        match value {
            GsmCbs::Noraml => 0,
            GsmCbs::ImmediateBroadcast => 1,
            GsmCbs::HighPriority => 2,
            GsmCbs::Reseverd => 3,
            GsmCbs::PriorityBackground => 4,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ansi136 {
    #[default]
    Buld = 0,
    Noraml = 1,
    Urgent = 2,
    VeryUrgent = 3,
}

impl From<u8> for Ansi136 {
    fn from(value: u8) -> Self {
        match value {
            0 => Ansi136::Buld,
            1 => Ansi136::Noraml,
            2 => Ansi136::Urgent,
            3 => Ansi136::VeryUrgent,
            _ => Ansi136::Noraml,
        }
    }
}

impl From<Ansi136> for u8 {
    fn from(value: Ansi136) -> Self {
        match value {
            Ansi136::Buld => 0,
            Ansi136::Noraml => 1,
            Ansi136::Urgent => 2,
            Ansi136::VeryUrgent => 3,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Is95 {
    #[default]
    Noraml = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}

impl From<u8> for Is95 {
    fn from(value: u8) -> Self {
        match value {
            0 => Is95::Noraml,
            1 => Is95::Interactive,
            2 => Is95::Urgent,
            3 => Is95::Emergency,
            _ => Is95::Noraml,
        }
    }
}

impl From<Is95> for u8 {
    fn from(value: Is95) -> Self {
        match value {
            Is95::Noraml => 0,
            Is95::Interactive => 1,
            Is95::Urgent => 2,
            Is95::Emergency => 3,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ansi41Cbs {
    #[default]
    Noraml = 0,
    Interactive = 1,
    Urgent = 2,
    Emergency = 3,
}

impl From<u8> for Ansi41Cbs {
    fn from(value: u8) -> Self {
        match value {
            0 => Ansi41Cbs::Noraml,
            1 => Ansi41Cbs::Interactive,
            2 => Ansi41Cbs::Urgent,
            3 => Ansi41Cbs::Emergency,
            _ => Ansi41Cbs::Noraml,
        }
    }
}

impl From<Ansi41Cbs> for u8 {
    fn from(value: Ansi41Cbs) -> Self {
        match value {
            Ansi41Cbs::Noraml => 0,
            Ansi41Cbs::Interactive => 1,
            Ansi41Cbs::Urgent => 2,
            Ansi41Cbs::Emergency => 3,
        }
    }
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
