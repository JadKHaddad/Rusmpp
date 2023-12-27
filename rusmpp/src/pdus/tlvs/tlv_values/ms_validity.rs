use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU8};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIo)]
pub struct MsValidity {
    pub validity_behaviour: MsValidityBehaviour,
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

#[async_trait::async_trait]
impl AsyncIoReadWithLength for MsValidity {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let validity_behaviour = MsValidityBehaviour::async_io_read(buf).await?;

        let validity_information_expected_length =
            length.saturating_sub(validity_behaviour.length());
        let validity_information = if validity_information_expected_length > 0 {
            Some(MsValidityInformation::async_io_read(buf).await?)
        } else {
            None
        };

        Ok(Self {
            validity_behaviour,
            validity_information,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIo)]
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

#[async_trait::async_trait]
impl AsyncIoRead for MsValidityInformation {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            units_of_time: UnitsOfTime::async_io_read(buf).await?,
            number_of_time_units: u16::async_io_read(buf).await?,
        })
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
