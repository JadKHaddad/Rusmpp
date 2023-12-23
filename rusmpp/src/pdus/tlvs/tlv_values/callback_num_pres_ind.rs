use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
}

impl CallbackNumPresInd {
    pub fn new(presentation: Presentation, screening: Screening) -> Self {
        Self {
            presentation,
            screening,
        }
    }
}

impl From<u8> for CallbackNumPresInd {
    fn from(value: u8) -> Self {
        Self {
            presentation: Presentation::from(value & 0b00000011),
            screening: Screening::from(value & 0b00001100),
        }
    }
}

impl From<CallbackNumPresInd> for u8 {
    fn from(value: CallbackNumPresInd) -> Self {
        u8::from(value.presentation) | u8::from(value.screening)
    }
}

impl IoLength for CallbackNumPresInd {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for CallbackNumPresInd {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for CallbackNumPresInd {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Presentation {
    PresentationAllowed = 0b00000000,
    PresentationRestricted = 0b00000001,
    NumberNotAvailable = 0b00000010,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Presentation {
    fn default() -> Self {
        Presentation::PresentationAllowed
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Screening {
    NotScreened = 0b00000000,
    VerivfiedAndPassed = 0b00000100,
    VerivfiedAndFailed = 0b00001000,
    NetworkProvided = 0b00001100,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Screening {
    fn default() -> Self {
        Screening::NotScreened
    }
}
