use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastAreaSuccess {
    #[default]
    InformationNotAvailable,
    ZeroToHundred(u8),
    Other(u8),
}

impl From<BroadcastAreaSuccess> for u8 {
    fn from(value: BroadcastAreaSuccess) -> Self {
        match value {
            BroadcastAreaSuccess::InformationNotAvailable => 255,
            BroadcastAreaSuccess::ZeroToHundred(value) => value,
            BroadcastAreaSuccess::Other(value) => value,
        }
    }
}

impl From<u8> for BroadcastAreaSuccess {
    fn from(value: u8) -> Self {
        match value {
            0..=100 => Self::ZeroToHundred(value),
            255 => Self::InformationNotAvailable,
            _ => Self::Other(value),
        }
    }
}

impl IoLength for BroadcastAreaSuccess {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for BroadcastAreaSuccess {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for BroadcastAreaSuccess {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
