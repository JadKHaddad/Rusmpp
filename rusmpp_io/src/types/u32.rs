use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoRead, IoReadError, IoReadable},
    write::{AsyncIoWritable, AsyncIoWrite, IoWritable, IoWrite},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const SIZE: usize = 4;

impl IoLength for u32 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u32 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        buf.write_u32(*self).await
    }
}

impl IoWrite for u32 {
    fn io_write(&self, buf: &mut IoWritable) -> std::io::Result<()> {
        buf.write_all(self.to_be_bytes().as_ref())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u32 {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(buf.read_u32().await?)
    }
}

impl IoRead for u32 {
    fn io_read(buf: &mut IoReadable) -> Result<Self, IoReadError> {
        let mut bytes = [0; SIZE];
        buf.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }
}
