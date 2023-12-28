use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU8};

use crate::io::read::{AsyncIoRead, AsyncIoReadable, IoReadError};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl BroadcastFrequencyInterval {
    pub fn new(unit: UnitOfTime, value: u16) -> Self {
        Self { unit, value }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for BroadcastFrequencyInterval {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let unit = UnitOfTime::async_io_read(buf).await?;
        let value = u16::async_io_read(buf).await?;

        Ok(Self { unit, value })
    }
}
