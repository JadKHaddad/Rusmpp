use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SIZE: usize = 2;

impl IoLength for u16 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u16 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        buf.write_u16(*self).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u16 {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(buf.read_u16().await?)
    }
}
