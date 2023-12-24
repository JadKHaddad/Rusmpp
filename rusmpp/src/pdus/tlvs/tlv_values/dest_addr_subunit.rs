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
pub enum DestAddrSubunit {
    Unknown = 0x00,
    MSDisplay = 0x01,
    MobileEquipment = 0x02,
    SmartCard = 0x03,
    ExternalUnit = 0x04,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for DestAddrSubunit {
    fn default() -> Self {
        DestAddrSubunit::Unknown
    }
}

impl IoLength for DestAddrSubunit {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DestAddrSubunit {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for DestAddrSubunit {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
