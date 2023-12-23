use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
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

impl IoLength for UnitOfTime {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for UnitOfTime {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for UnitOfTime {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl BroadcastFrequencyInterval {
    pub fn new(unit: UnitOfTime, value: u16) -> Self {
        Self { unit, value }
    }
}

impl IoLength for BroadcastFrequencyInterval {
    fn length(&self) -> usize {
        self.unit.length() + self.value.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for BroadcastFrequencyInterval {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.unit.async_io_write(buf).await?;
        self.value.async_io_write(buf).await?;

        Ok(())
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
