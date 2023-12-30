use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoReadLength, RusmppIoU8, RusmppIoWrite};

use crate::io::{length::IoLength, read::AsyncIoRead};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoReadLength,
)]
pub struct MsValidity {
    pub validity_behaviour: MsValidityBehaviour,
    #[rusmpp_io_read(length=(length - all_before))]
    pub validity_information: Option<MsValidityInformation>,
}

impl MsValidity {
    pub fn new(
        validity_behaviour: MsValidityBehaviour,
        validity_information: Option<MsValidityInformation>,
    ) -> Self {
        Self {
            validity_behaviour,
            validity_information,
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoRead,
)]
pub struct MsValidityInformation {
    pub units_of_time: UnitsOfTime,
    pub number_of_time_units: u16,
}

impl MsValidityInformation {
    pub fn new(units_of_time: UnitsOfTime, number_of_time_units: u16) -> Self {
        Self {
            units_of_time,
            number_of_time_units,
        }
    }
}

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
pub enum MsValidityBehaviour {
    StoreIndefinitely = 0,
    PowerDown = 1,
    ValidUntilRegistrationAreaChanges = 2,
    DisplayOnly = 3,
    RelativeTimePeriod = 4,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for MsValidityBehaviour {
    fn default() -> Self {
        MsValidityBehaviour::StoreIndefinitely
    }
}

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
pub enum UnitsOfTime {
    Seconds = 0b00000000,
    Minutes = 0b00000001,
    Hours = 0b00000010,
    Days = 0b00000011,
    Weeks = 0b00000100,
    Months = 0b00000101,
    Years = 0b00000110,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for UnitsOfTime {
    fn default() -> Self {
        UnitsOfTime::Seconds
    }
}
