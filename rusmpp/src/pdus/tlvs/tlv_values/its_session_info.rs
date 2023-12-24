use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}

impl IoLength for ItsSessionInfo {
    fn length(&self) -> usize {
        self.session_number.length() + self.sequence_number.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for ItsSessionInfo {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.session_number.async_io_write(buf).await?;
        self.sequence_number.async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for ItsSessionInfo {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            session_number: u8::async_io_read(buf).await?,
            sequence_number: u8::async_io_read(buf).await?,
        })
    }
}
