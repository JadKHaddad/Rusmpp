use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoU8, RusmppIoWrite};

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
)]
pub enum UnitOfTime {
    AsFrquentlyAsPossible = 0x00,
    Seconds = 0x08,
    Minutes = 0x09,
    Hours = 0x0A,
    Days = 0x0B,
    Weeks = 0x0C,
    Months = 0x0D,
    Years = 0x0E,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for UnitOfTime {
    fn default() -> Self {
        UnitOfTime::AsFrquentlyAsPossible
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl BroadcastFrequencyInterval {
    pub fn new(unit: UnitOfTime, value: u16) -> Self {
        Self { unit, value }
    }
}
