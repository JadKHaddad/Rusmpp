use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

impl From<u8> for NumberOfMessages {
    fn from(value: u8) -> Self {
        match value {
            0..=99 => NumberOfMessages::Allowed(value),
            _ => NumberOfMessages::Other(value),
        }
    }
}

impl From<NumberOfMessages> for u8 {
    fn from(value: NumberOfMessages) -> Self {
        match value {
            NumberOfMessages::Allowed(value) => value,
            NumberOfMessages::Other(value) => value,
        }
    }
}

impl IoLength for NumberOfMessages {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for NumberOfMessages {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for NumberOfMessages {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
