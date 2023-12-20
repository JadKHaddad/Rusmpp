use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Npi {
    Unknown = 0b00000000,
    Isdn = 0b00000001,
    Data = 0b00000011,
    Telex = 0b00000100,
    LandMobile = 0b00000110,
    National = 0b00001000,
    Private = 0b00001001,
    Ermes = 0b00001010,
    Internet = 0b00001110,
    WapClientId = 0b00010010,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Npi {
    fn default() -> Self {
        Npi::Unknown
    }
}

impl IoLength for Npi {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for Npi {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for Npi {
    type Error = std::io::Error;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, Self::Error> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
