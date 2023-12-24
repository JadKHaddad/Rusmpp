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
pub enum DeliveryFailureReason {
    DestinationUnavailable = 0,
    DestinationAddressInvalid = 1,
    PermanentNetworkError = 2,
    TemporaryNetworkError = 3,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for DeliveryFailureReason {
    fn default() -> Self {
        DeliveryFailureReason::DestinationUnavailable
    }
}

impl IoLength for DeliveryFailureReason {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DeliveryFailureReason {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for DeliveryFailureReason {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
